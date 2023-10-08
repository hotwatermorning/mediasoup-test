<script lang="ts">
  import { onMount } from "svelte";
  import { Button, CircularProgress } from "~/component";
  import { getClientSettingContext } from "~/context/ClientSettingProvider.svelte";
  import { countGrapheme } from "~/lib/text";
  import type { DOMEvent } from "~/util/domEvents";
  import { createLoadingGuard } from "~/util/loadingGuard";

  const { clientSettingStore, setName, setCameraId, setMicId, isValidSetting } = getClientSettingContext();
  const nameLengthLimit = 25;

  type Device = {
    id: string;
    name: string;
  };

  const { loadingGuard, loadingStore } = createLoadingGuard();

  let cameraList: Device[] = [];
  let micList: Device[] = [];
  let selectedCameraId: string;
  let selectedMicId: string;
  let previewVideoElement: HTMLVideoElement | undefined = undefined;;

  let name = "";
  $: count = countGrapheme(name);
  $: counterText = `現在：${count}文字/${nameLengthLimit}文字`;
  $: name = name.replaceAll("\n", "");

  onMount(async () => {
    loadingGuard(async () => {
      const devices = await navigator.mediaDevices.enumerateDevices();

      for(const d of devices) {
        if(d.kind === "audioinput") {
          micList.push({
            id: d.deviceId,
            name: d.label,
          });
        } else if(d.kind === "videoinput") {
          cameraList.push({
            id: d.deviceId,
            name: d.label,
          });
        }
      }

      if(micList.length > 0) {
        onChangeMic(micList[0].id);
      }
      if(cameraList.length > 0) {
        onChangeCamera(cameraList[0].id);
      }
    });
  });

  const onChangeMic = (deviceId: string) => {
    if(deviceId === "" || deviceId === selectedMicId) {
      return;
    }

    console.log(`onChangeMic: ${deviceId}`);
    selectedMicId = deviceId;
  };

  const onChangeCamera = async (deviceId: string) => {
    if(deviceId === "" || deviceId === selectedCameraId) {
      return;
    }

    console.log(`onChangeCamera: ${deviceId}`);
    selectedCameraId = deviceId;

    const media = await navigator.mediaDevices.getUserMedia({
      video: {
        deviceId
      }
    });

    if(previewVideoElement === undefined) {
      return;
    }

    previewVideoElement.srcObject = media;
    previewVideoElement.autoplay = true;
  };

  const onSave = async () => {
    setCameraId(selectedCameraId);
    setMicId(selectedMicId);
    setName(name);

    if(previewVideoElement) {
      previewVideoElement.srcObject = null;
    }
  };
</script>

<div class="layout">
  <h1>ユーザー設定</h1>
  <section>
    <h2 class="section-title">表示名</h2>
    <div class="layout-input">
      <input type="text" class="input" bind:value={name} />
      <div class={`counter ${count > nameLengthLimit ? "invalid" : ""}`}>
        {counterText}
      </div>
    </div>
  </section>
  <section>
    {#if $loadingStore}
      <CircularProgress />
    {:else}
      <h2 class="section-title">デバイス設定</h2>
      <!-- svelte-ignore a11y-media-has-caption -->
      <div class="layout-device-selection">
        <div class="layout-device-controls">
          <label>
            <div class="device-control-label">マイク</div>
            <select class="device-select-control" on:change={(e) => onChangeMic(e.currentTarget.value)}>
              {#each micList as c}
                <option value={c.id}>{c.name}</option>
              {/each}
            </select>
          </label>
          <label>
            <div class="device-control-label">カメラ</div>
            <select class="device-select-control" on:change={(e) => onChangeCamera(e.currentTarget.value)}>
              {#each cameraList as c}
                <option value={c.id}>{c.name}</option>
              {/each}
            </select>
          </label>
        </div>
        <div class="layout-preview-video">
          <video class="preview-video" muted bind:this={previewVideoElement} />
        </div>
      </div>
    {/if}
  </section>
  <div class="layout-dialog-controls">
    <Button
      on:click={onSave}
      primary={true}
      disabled={count === 0 || count > nameLengthLimit}>設定して参加</Button
    >
  </div>
</div>

<style>
  .layout {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 20px;
    width: 80vw;
    max-width: 600px;
  }

  .section-title {
    height: 34px;
    width: 100%;
    text-align: center;
    font-family: Noto Sans CJK JP;
    font-size: 24px;
    font-style: normal;
    font-weight: 700;
    line-height: 34px;
  }

  .input {
    width: 100%;
    color: #3a3e49;
    font-family: Noto Sans CJK JP;
    font-size: 16px;
    font-style: normal;
    font-weight: 400;
    line-height: 20px;
    border-radius: 2px;
    border: 1px solid #dbdbdb;
    background: var(--ffffff, #fff);
    box-sizing: border-box;
    padding: 10px;
    text-align: center;
  }

  .counter {
    text-align: right;
    font-size: 12px;

    &.invalid {
      color: red;
    }
  }

  .layout-device-selection {
    display: flex;
    gap: 20px;
  }

  .layout-device-controls {
    display: flex;
    flex-direction: column;
    gap: 20px;

    & > label {
      display: flex;
      gap: 10px;
      align-items: center;
    }
  }

  .device-control-label {
    flex-shrink: 0;
    width: 50px;
  }

  .layout-dialog-controls {
    display: flex;
    justify-content: center;
    gap: 20px;
  }

  .device-select-control {
    display: block;
    width: 100%;
  }

  .layout-preview-video {
    display: flex;
    justify-content: end;
    width: 240px;
    height: 180px;
    background-color: #232323;
  }

  .preview-video {
    width: 100%;
    height: auto;
    object-fit: contain;
  }
</style>
