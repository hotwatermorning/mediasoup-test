<script lang="ts">
  import { VideoChatManager, type ParticipantInfo, init } from "~/lib/api";
  import CameraOn from "~/asset/CameraOn.svg?component";
  import CameraOff from "~/asset/CameraOff.svg?component";
  import MicOn from "~/asset/MicOn.svg?component";
  import MicOff from "~/asset/MicOff.svg?component";
  import { getClientSettingContext } from "~/context/ClientSettingProvider.svelte";

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
      await init(name, mgr, $clientSettingStore.micId, $clientSettingStore.cameraId, node);
    })();

    return {
      destroy: () => {
        console.log("destroyed");
      },
      update: () => {
        console.log("updated");
      },
    };
  }

  const onBind = (node: HTMLVideoElement, id: string) => {
    if (!mgr) {
      return;
    }

    node.onloadedmetadata = () => {
      node.play();
    };

    mgr.bind(id, node);
  }

  const onInitSelfVideo_ = (node: HTMLVideoElement) => {
    console.log("onInitSelfVideo_");

    onInit(node);
  };

  const onBindParticipantElement_ = (node: HTMLVideoElement, id: string) => {
    console.log("onBindParticipantElement_");

    onBind(node, id);
  };

  const onChangeCameraStatus = () => {
    if(!mgr) { return; }

    mgr.setCarameraEnabled(mgr.isCameraEnabled() === false);
    isCameraEnabled = mgr.isCameraEnabled();
    console.log("onChangeCameraStatus");
  };

  const onChangeMicStatus = () => {
    if(!mgr) { return; }

    mgr.setMicEnabled(mgr.isMicEnabled() === false);
    isMicEnabled = mgr.isMicEnabled();
    console.log("onChangeMicStatus");
  };

</script>

<div class="layout">
  <div class="video-wrapper">
    <figure>
      <figcaption>You ({name})</figcaption>
      <div class="layout-video-controls">
        <video id="preview-send" muted use:onInitSelfVideo_ />
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
  {#each participants as { id, name } (id)}
    <div class="video-wrapper">
      <figure>
        <figcaption>{name}</figcaption>
        <video muted use:onBindParticipantElement_={id} />
      </figure>
    </div>
  {/each}
</div>

<style>
  .layout {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  figure {
    display: block;
    margin-block: unset;
    margin-inline: unset;
  }

  .video-wrapper {
    max-width: 1200px;
    width: 100%;
    height: auto;
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
    transition:
      background-color 0.1s ease
      visibility 0.1 ease;

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
    margin-top: 0 auto;
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
      border: #FFFFFF80 6px solid;
    }
  }

  video {
    height: 100%;
    width: 100%;
  }

  figcaption {
    color: white;
    font-size: 1.5rem;
  }
</style>