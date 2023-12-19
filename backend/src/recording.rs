use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::process::Child;
use std::process::ChildStderr;
use std::process::Stdio;
use std::thread;
use std::time;

use crate::room::media_codecs;
use crate::util::*;
use mediasoup::plain_transport::*;
use mediasoup::prelude::*;
use mediasoup::rtp_parameters::RtpCodecCapabilityFinalized;

use std::process::Command;

#[derive(Default, Debug)]
pub struct Recorder {
    pub audio_transport: Option<PlainTransport>,
    pub video_transport: Option<PlainTransport>,
    pub audio_consumer: Option<Consumer>,
    pub video_consumer: Option<Consumer>,
    pub process: Option<Child>,
    pub reader: Option<BufReader<ChildStderr>>,
    pub is_recording: bool,
    pub filename: String,
}

impl Recorder {
    pub async fn new(
        router: &Router,
        audio_producer: Option<&Producer>,
        video_producer: Option<&Producer>,
    ) -> Result<Self, String> {
        let mut tmp_self = Recorder::default();

        // audio
        if let Some(ap) = audio_producer {
            let mut transport_options = PlainTransportOptions::new(ListenIp {
                ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
                announced_ip: None,
            });
            transport_options.comedia = false;
            transport_options.rtcp_mux = false;

            let transport = router
                .create_plain_transport(transport_options)
                .await
                .map_err(|error| format!("Failed to create audio transport: {error}"))?;

            let remote_params = PlainTransportRemoteParameters {
                ip: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
                port: Some(get_env::<u16>("AUDIO_RECORDING_PORT_RTP").expect("should be defined")),
                rtcp_port: Some(
                    get_env::<u16>("AUDIO_RECORDING_PORT_RTCP").expect("should be defined"),
                ),
                srtp_parameters: None,
            };

            transport
                .connect(remote_params)
                .await
                .map_err(|error| format!("Failed to connect audio transport: {error}"))?;

            log::debug!("audio transport tuple: {:?}", &transport.tuple());
            log::debug!("audio transport rtcp tuple: {:?}", &transport.rtcp_tuple());

            let src_cap = convert_rtp_capabilities(router.rtp_capabilities());
            log::debug!("audio capabilities: {:?}", &src_cap);

            let mut cap = RtpCapabilities::default();

            cap.header_extensions = src_cap.header_extensions.clone();

            let codec = media_codecs()[0].clone();
            cap.codecs.push(codec);

            let mut consume_options = ConsumerOptions::new(ap.id(), cap);
            consume_options.paused = true;

            let consumer = transport
                .consume(consume_options)
                .await
                .map_err(|error| format!("Failed to consume audio transport: {error}"))?;

            tmp_self.audio_transport = Some(transport);
            tmp_self.audio_consumer = Some(consumer);
        }

        // video
        if let Some(vp) = video_producer {
            let mut transport_options = PlainTransportOptions::new(ListenIp {
                ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
                announced_ip: None,
            });
            transport_options.comedia = false;
            transport_options.rtcp_mux = false;

            let transport = router
                .create_plain_transport(transport_options)
                .await
                .map_err(|error| format!("Failed to create video transport: {error}"))?;

            // transport.enable_trace_event(vec![TransportTraceEventType::Probation]);
            // transport.on_trace(Arc::new(|ev: &TransportTraceEventData| match ev {
            //     TransportTraceEventData::Probation {
            //         timestamp,
            //         direction,
            //         info,
            //     } => {
            //         log::debug!("Probation: {:?}", &info);
            //     }
            //     TransportTraceEventData::Bwe {
            //         timestamp,
            //         direction,
            //         info,
            //     } => {
            //         log::debug!("Bwe: {:?}", &info);
            //     }
            // }));

            let remote_params = PlainTransportRemoteParameters {
                ip: Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
                port: Some(get_env::<u16>("VIDEO_RECORDING_PORT_RTP").expect("should be defined")),
                rtcp_port: Some(
                    get_env::<u16>("VIDEO_RECORDING_PORT_RTCP").expect("should be defined"),
                ),
                srtp_parameters: None,
            };

            transport
                .connect(remote_params)
                .await
                .map_err(|error| format!("Failed to connect video transport: {error}"))?;

            log::debug!("video transport tuple: {:?}", &transport.tuple());
            log::debug!("video transport rtcp tuple: {:?}", &transport.rtcp_tuple());

            let src_cap = convert_rtp_capabilities(router.rtp_capabilities());
            log::debug!("video capabilities: {:?}", &src_cap);

            let mut cap = RtpCapabilities::default();

            cap.header_extensions = src_cap.header_extensions.clone();

            let codec = media_codecs()[1].clone();
            cap.codecs.push(codec);

            let mut consume_options = ConsumerOptions::new(vp.id(), cap);
            consume_options.paused = true;

            let consumer = transport
                .consume(consume_options)
                .await
                .map_err(|error| format!("Failed to consume video transport: {error}"))?;

            tmp_self.video_transport = Some(transport);
            tmp_self.video_consumer = Some(consumer);
        }

        Ok(tmp_self)
    }

    pub async fn start_recording(&mut self, output_name: &str) -> Result<(), String> {
        self.start_recording_process(&format!("{}_tmp", output_name))
            .await?;

        if let Some(c) = self.audio_consumer.as_ref() {
            c.resume()
                .await
                .map_err(|e| format!("Failed to start audio consumer: {e}"))?;

            log::debug!("redume audio consumer");
        }

        if let Some(c) = self.video_consumer.as_ref() {
            c.resume()
                .await
                .map_err(|e| format!("Failed to start video consumer: {e}"))?;

            log::debug!("redume video consumer");
        }

        self.is_recording = true;
        self.filename = output_name.to_string();

        Ok(())
    }

    async fn start_recording_process(&mut self, output_name: &str) -> Result<(), String> {
        let cmd_program = "ffmpeg";
        let sdp = "./profiles/input-h264.sdp";

        let cmd_output_path = format!("./recordings/{}.mp4", output_name);

        let cmd_format = vec!["-f", "mp4", "-strict", "experimental"];

        // Run process
        let cmd_args = [
            vec![
                "-protocol_whitelist",
                "file,rtp,udp",
                "-probesize",
                "50M",
                "-fflags",
                "+genpts",
                "-i",
                sdp,
            ],
            cmd_format,
            vec!["-y", cmd_output_path.as_ref()],
        ]
        .concat();

        log::info!("spawn ffmpeg: {:?}", &cmd_program);

        let mut proc = Command::new(cmd_program)
            .args(cmd_args)
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|error| format!("Failed to consume audio transport: {error}"))?;

        log::info!("get ffmpeg handle");

        let stderr = proc
            .stderr
            .take()
            .ok_or("Failed to take stdout".to_owned())?;
        log::info!("take stderr");

        let mut r = BufReader::with_capacity(10000000, stderr);

        log::info!("get buf reader");

        loop {
            log::info!("get ffmpeg output");
            let mut line = String::new();
            let result = r.read_line(&mut line);
            if let Err(e) = result {
                return Err(format!("Failed to read line: {e}"));
            }

            if let Ok(0) = result {
                return Err("FFmpeg is quit".to_owned());
            }

            log::debug!("line: {}", &line);
            if line.starts_with("ffmpeg version") {
                break;
            }
        }

        thread::spawn(move || {
            log::info!("read lines.");
            loop {
                let mut buf = String::new();
                let result = r.read_line(&mut buf);
                if result.is_err() || result.unwrap_or(1) == 0 {
                    break;
                }

                log::info!("{}", &buf);
            }
        });

        log::debug!("ffmpeg has been started.");

        self.process = Some(proc);
        // self.reader = Some(r);
        Ok(())
    }

    pub fn stop_ffmpeg(&mut self) -> Result<(), String> {
        let proc = std::mem::replace(&mut self.process, None);

        log::info!("thread started.");
        let Some(mut c) = proc else {
            return Err("proc is none".to_owned());
        };

        if let Some(stream) = c.stdin.as_mut() {
            stream.write(b"q\n");
            stream.flush();
        }

        let _ = c
            .wait()
            .map_err(|e| format!("FFmpeg failed to exit: {e}"))?;

        Ok(())
    }

    pub async fn stop_recording(&mut self) -> Result<(), String> {
        if self.is_recording == false {
            return Ok(());
        }

        self.stop_ffmpeg();
        let filename = self.filename.clone();

        let src_path = format!("./recordings/{}_tmp.mp4", &filename);
        let dest_path = format!("./recordings/{}.mp4", &filename);
        let _ = std::fs::rename(src_path, dest_path);

        if let Some(c) = self.audio_consumer.as_ref() {
            c.pause()
                .await
                .map_err(|e| "Failed to pause audio consumer: {e}".to_owned())?;
        }

        if let Some(c) = self.video_consumer.as_ref() {
            c.pause()
                .await
                .map_err(|e| "Failed to pause video consumer: {e}".to_owned())?;
        }

        Ok(())
    }
}

fn convert_rtp_codec_capability(src: &RtpCodecCapabilityFinalized) -> RtpCodecCapability {
    match src {
        RtpCodecCapabilityFinalized::Audio {
            mime_type,
            preferred_payload_type,
            clock_rate,
            channels,
            parameters,
            rtcp_feedback,
        } => RtpCodecCapability::Audio {
            mime_type: mime_type.clone(),
            preferred_payload_type: Some(preferred_payload_type.clone()),
            clock_rate: clock_rate.clone(),
            channels: channels.clone(),
            parameters: parameters.clone(),
            rtcp_feedback: rtcp_feedback.clone(),
        },
        RtpCodecCapabilityFinalized::Video {
            mime_type,
            preferred_payload_type,
            clock_rate,
            parameters,
            rtcp_feedback,
        } => RtpCodecCapability::Video {
            mime_type: mime_type.clone(),
            preferred_payload_type: Some(preferred_payload_type.clone()),
            clock_rate: clock_rate.clone(),
            parameters: parameters.clone(),
            rtcp_feedback: rtcp_feedback.clone(),
        },
        _ => panic!("Unknown type"),
    }
}

fn convert_rtp_capabilities(src: &RtpCapabilitiesFinalized) -> RtpCapabilities {
    let mut dest = RtpCapabilities::default();

    dest.header_extensions = src.header_extensions.clone();

    dest.codecs = src
        .codecs
        .iter()
        .map(convert_rtp_codec_capability)
        .collect();

    dest
}
