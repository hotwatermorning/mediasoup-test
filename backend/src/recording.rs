use std::io::BufRead;
use std::io::BufReader;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::num::NonZeroU32;
use std::num::NonZeroU8;
use std::process::Child;
use std::process::Stdio;

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
    pub ffmpeg_process: Option<Child>,
    pub is_recording: bool,
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
            let codec = RtpCodecCapability::Audio {
                mime_type: MimeTypeAudio::Opus,
                preferred_payload_type: Some(111),
                clock_rate: NonZeroU32::new(48000).unwrap(),
                channels: NonZeroU8::new(2).unwrap(),
                parameters: Default::default(),
                rtcp_feedback: Default::default(),
            };
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
            let codec = RtpCodecCapability::Video {
                mime_type: MimeTypeVideo::Vp8,
                preferred_payload_type: Some(96),
                clock_rate: NonZeroU32::new(90000).unwrap(),
                parameters: Default::default(),
                rtcp_feedback: Default::default(),
            };
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
        // self.start_recording_process(output_name).await?;

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

        Ok(())
    }

    //     fn get_sdp_string(kind: MediaKind, params: RtpParameters) -> String {
    //       let codec = params.codecs.get(0);
    //       let Some(codec) = codec else {
    //         return "".to_owned();
    //       };
    //
    //       match codec {
    //         RtpCodecParameters::Audio { mime_type, payload_type, clock_rate, channels, parameters, rtcp_feedback } => {
    //           format!("\nm=audio {} RTP/AVPF {}}\n\
    //             a=rtcp:{}\n\
    //             a=rtpmap:{} {videoCodecInfo.codecName}/{videoCodecInfo.clockRate}\n\
    //             a=rtpmap:111 opus/48000/2",
    //             get_env::<u16>("AUDIO_RECORDING_PORT_RTP").unwrap(),
    //             payload_type,
    //             get_env::<u16>("AUDIO_RECORDING_PORT_RTP").unwrap(),
    //             payload_type,
    //
    //              3).as_str();
    //           }
    //         RtpCodecParameters::Video { mime_type, payload_type, clock_rate, parameters, rtcp_feedback } => {
    //           format!("\nm=video {} RTP/AVPF 96\n\
    //             a=rtcp:{}\n\
    //             a=rtpmap:96 VP8/90000",
    //             1, 2).as_str();
    //           }
    //       }
    //     //   return {
    //     //     payloadType: rtpParameters.codecs[0].payloadType,
    //     //     codecName: rtpParameters.codecs[0].mimeType.replace(`${kind}/`, ''),
    //     //     clockRate: rtpParameters.codecs[0].clockRate,
    //     //     channels: kind === 'audio' ? rtpParameters.codecs[0].channels : undefined
    //     //   };
    //     }
    //
    //     fn create_sdp(audio_rtp: Option<RtpParameters>, video_rtp: Option<RtpParameters>) -> String {
    //       let mut sdp = format!("data:application/sdp;charset=UTF-8,v=0\n\
    //         o=- 0 0 IN IP4 127.0.0.1\n\
    //         s=-\n\
    //         c=IN IP4 127.0.0.1\n\
    //         t=0 0\n");
    //
    //       if let Some(r) = audio_rtp {
    //         sdp += format!("\nm=audio {} RTP/AVPF {}}\n\
    //           a=rtcp:{}\n\
    //           a=rtpmap:111 opus/48000/2\n\
    //           a=fmtp:111 minptime=10;useinbandfec=1",
    //           get_env::<u16>("AUDIO_RECORDING_PORT_RTP").unwrap(),
    //           r, 3).as_str();
    //       }
    //
    //       if let Some(r) = video_rtp {
    //         sdp += format!("\nm=video {} RTP/AVPF 96\n\
    //           a=rtcp:{}\n\
    //           a=rtpmap:96 VP8/90000",
    //           1, 2).as_str();
    //       }
    //
    //       sdp
    //     };

    async fn start_recording_process(&mut self, output_name: &str) -> Result<(), String> {
        let cmd_program = "ffmpeg";
        let cmd_input_path = "./profiles/input-h264.sdp";

        let cmd_output_path = format!("./recordings/{}.mp4", output_name);

        let cmd_format = vec!["-f", "webm", "-flags", "+global_header"];
        // let cmd_format = vec!["-f", "mp4", "-strict", "experimental"];

        let mut cmd_codec = Vec::<&str>::new();

        if self.audio_transport.is_some() {
            cmd_codec.extend(["-map", "0:a:0", "-c:a", "copy"]);
        }
        if self.video_transport.is_some() {
            cmd_codec.extend(["-map", "0:v:0", "-c:v", "copy"]);

            // // "-strict experimental" is required to allow storing
            // // OPUS audio into MP4 container
            // cmdFormat = "-f mp4 -strict experimental";
        }

        // let sdp = create_sdp(self.audio_consumer.map(|c| c.rtp_parameters()), self.video_consumer.map(|c| c.rtp_parameters()));
        let sdp = "./profiles/input-vp8.sdp";

        // Run process
        let cmd_args = [
            vec![
                "-nostdin",
                "-protocol_whitelist",
                "file,rtp,udp",
                "-loglevel",
                "debug",
                // "-analyzeduration",
                // "5M",
                // "-probesize",
                // "5M",
                "-fflags",
                "+genpts",
                "-i",
                sdp,
            ],
            cmd_codec,
            cmd_format,
            vec!["-y", cmd_output_path.as_ref()],
        ]
        .concat();

        log::info!("spawn ffmpeg");

        let proc = Command::new(cmd_program)
            .args(cmd_args)
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| format!("Failed to consume audio transport: {error}"))?;

        log::info!("get ffmpeg handle");

        //         let stderr = proc.stderr.take().expect("Failed to take stdout");
        //
        //         log::info!("take stderr");
        //
        //         let mut r = BufReader::new(stderr);
        //
        //         log::info!("get buf reader");
        //
        //         loop {
        //             log::info!("get ffmpeg output");
        //             let mut line = String::new();
        //             let result = r.read_line(&mut line);
        //             if let Err(e) = result {
        //                 return Err(format!("Failed to read line: {e}"));
        //             }
        //
        //             if let Ok(0) = result {
        //                 return Err("FFmpeg is quit".to_owned());
        //             }
        //
        //             if line.starts_with("ffmpeg version") {
        //                 break;
        //             }
        //         }
        //
        //         log::debug!("ffmpeg has been started.");
        //
        //         std::mem::drop(r);
        //         proc.stderr = Some(stderr);
        self.ffmpeg_process = Some(proc);
        Ok(())
    }

    pub async fn stop_recording(&mut self) -> Result<(), String> {
        if self.is_recording == false {
            return Ok(());
        }

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

        if let Some(c) = self.ffmpeg_process.as_mut() {
            let mut kill = Command::new("kill")
                .args(["-s", "SIGINT", &c.id().to_string()])
                .spawn()
                .map_err(|e| "Failed to spawn kill command: {e}".to_owned())?;

            kill.wait()
                .map_err(|e| "Failed to kill ffmpeg: {e}".to_owned())?;
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
