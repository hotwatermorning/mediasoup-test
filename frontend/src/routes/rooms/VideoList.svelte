<script lang="ts">
  import { VideoChatManager, type ParticipantInfo, init } from "~/lib/api";
  import CameraOn from "~/asset/CameraOn.svg?component";
  import CameraOff from "~/asset/CameraOff.svg?component";
  import MicOn from "~/asset/MicOn.svg?component";
  import MicOff from "~/asset/MicOff.svg?component";
  import { getClientSettingContext } from "~/context/ClientSettingProvider.svelte";
  import { goto } from "$app/navigation";

  const { clientSettingStore } = getClientSettingContext();

  export let name = $clientSettingStore.name;
  let mgr: VideoChatManager | undefined = undefined;
  let isCameraEnabled = false;
  let isMicEnabled = false;
  let participants: ParticipantInfo[] = [];

  const onInit = (node: HTMLVideoElement) => {
    mgr = new VideoChatManager(() => {
      if (mgr) {
        isCameraEnabled = mgr.isCameraEnabled();
        isMicEnabled = mgr.isMicEnabled();
        participants = mgr.getParticipants();
      }
    });

    node.onloadedmetadata = () => {
      node.play();
    };

    (async () => {
      await init(
        name,
        mgr,
        $clientSettingStore.micId,
        $clientSettingStore.cameraId,
        node,
        (url) => {
          goto(url, { replaceState: true });
        }
      );
    })();

    return {
      destroy: () => {
        console.log("destroyed");
      },
      update: () => {
        console.log("updated");
      },
    };
  };

  const onBind = (node: HTMLVideoElement, id: string) => {
    if (!mgr) {
      return;
    }

    node.onloadedmetadata = () => {
      node.play();
    };

    mgr.bind(id, node);
  };

  const onInitSelfVideo_ = (node: HTMLVideoElement) => {
    console.log("onInitSelfVideo_");

    onInit(node);
  };

  const onBindParticipantElement_ = (node: HTMLVideoElement, id: string) => {
    console.log("onBindParticipantElement_");

    onBind(node, id);
  };

  const onChangeCameraStatus = () => {
    if (!mgr) {
      return;
    }

    mgr.setCarameraEnabled(mgr.isCameraEnabled() === false);
    isCameraEnabled = mgr.isCameraEnabled();
    console.log("onChangeCameraStatus");
  };

  const onChangeMicStatus = () => {
    if (!mgr) {
      return;
    }

    mgr.setMicEnabled(mgr.isMicEnabled() === false);
    isMicEnabled = mgr.isMicEnabled();
    console.log("onChangeMicStatus");
  };

  const getVideoLayoutStyle = (numParticipants: number) => {
    if(numParticipants + 1 <= 1) {
      return "video-layout-for-1";
    } else if(numParticipants + 1 <= 4) {
      return "video-layout-for-4";
    } else if(numParticipants + 1 <= 9) {
      return "video-layout-for-9";
    } else if(numParticipants + 1 <= 16) {
      return "video-layout-for-16";
    } else {
      return "video-layout-for-many";
    }
  };

  const startRecording = async () => {
    mgr?.startRecording(new Date().valueOf().toString());
  };

  const stopRecording = async () => {
    mgr?.stopRecording();
  };

</script>

<div class="layout">
  <button on:click={startRecording}>録画開始</button>
  <button on:click={stopRecording}>録画停止</button>
  <div class={getVideoLayoutStyle(participants.length)}>
    <div class="video-wrapper">
      <div class="dummy">
        <figure>
          <figcaption>You ({name})</figcaption>
          <div class="layout-video-controls">
            <video
              id="preview-send"
              muted
              use:onInitSelfVideo_
              poster={"/poster.svg"}
            />
            <div class="overlay">
              <div class="video-controls">
                <button class="control-button" on:click={onChangeMicStatus}>
                  {#if isMicEnabled}
                    <MicOn />
                  {:else}
                    <MicOff />
                  {/if}
                </button>
                <button class="control-button" on:click={onChangeCameraStatus}>
                  {#if isCameraEnabled}
                    <CameraOn />
                  {:else}
                    <CameraOff />
                  {/if}
                </button>
              </div>
            </div>
          </div>
        </figure>
      </div>
    </div>
    {#each participants as { id, name } (id)}
      <div class="video-wrapper">
        <div class="dummy">
          <figure>
            <figcaption>{name}</figcaption>
            <video
              use:onBindParticipantElement_={id}
              poster={"/poster.svg"}
            />
          </figure>
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .layout {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    height: 100%;
  }

  .video-layout-for-1 {
    display: flex;
    width: 100%;
    justify-content: center;
    gap: 10px;
    height: 100%;
    width: 100%;
  }

  .video-layout-for-4 {
    display: grid;
    width: 100%;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    column-gap: 10px;
    row-gap: 10px;
  }

  .video-layout-for-9 {
    display: grid;
    width: 100%;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    column-gap: 10px;
    row-gap: 10px;
  }

  .video-layout-for-16 {
    display: grid;
    width: 100%;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    column-gap: 10px;
    row-gap: 10px;
  }

  .video-layout-for-many {
    display: grid;
    width: 100%;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    column-gap: 10px;
    row-gap: 10px;
  }

  figure {
    display: block;
    margin-block: unset;
    margin-inline: unset;
  }

  .video-wrapper {
    max-width: 800px;
    width: 100%;
    height: auto;
  }

  .dummy {
    container: clayout / inline-size;
  }

  .layout-video-controls {
    width: 100%;
    height: auto;
    position: relative;
  }

  .overlay {
    display: none;
    visibility: hidden;
  }

  .layout-video-controls:hover .overlay {
    display: flex;
    visibility: visible;
    flex-direction: column;
    justify-content: end;
    z-index: 1;
    color: white;
    visibility: visible;
    position: absolute;
    top: 0px;
    left: 0px;
    width: 100%;
    height: 100%;
    transition: background-color 0.1s ease visibility 0.1 ease;

    &:hover {
      background-color: #00000040;
    }
  }

  .video-controls {
    height: 100px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 60px;
  }

  @container (max-width: 500px) {
    .video-controls {
      gap: 45px;
      height: 75px;
    }
  }

  .video-controls > button {
    display: block;
    height: 60px;
    width: 60px;
    border-radius: 60px;
    background: none;
    padding: 0;
    margin: 0;
    border: none;
    border: transparent 6px solid;
    box-sizing: content-box;

    transition: border 0.2s ease;

    &:hover {
      border: #ffffff80 6px solid;
    }
  }

  @container (max-width: 500px) {
    .video-controls > button {
      height: 45px;
      width: 45px;
      border-radius: 45px;
    }
  }

  video {
    height: 100%;
    width: 100%;
    aspect-ratio: 4 / 3;
    object-fit: cover;

    &:disabled {
      visibility: hidden;
    }
  }

  figcaption {
    color: white;
    font-size: 1.5rem;
    text-align: center;
    line-height: normal;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 1;
    -webkit-box-orient: vertical;
  }

  @container (max-width: 500px) {
    figcaption {
      font-size: 0.8rem
    }
  }
</style>
