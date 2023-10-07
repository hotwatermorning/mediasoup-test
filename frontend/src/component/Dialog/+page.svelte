<script lang="ts">
  import { onMount } from 'svelte';
  import CloseIcon from '~/asset/Close.svg?component';

  export let onClose: () => void;
  export let showCloseButton = false;
  export let closeWhenClickingOutside = true;
  let dialog: HTMLDialogElement;
  let contents: HTMLDivElement;

  const onClick = (ev: MouseEvent) => {
    if (contents.contains(ev.target as Node)) {
      ev.stopPropagation();
      return;
    }

    onClose();
  };

  onMount(() => {
    dialog.showModal();
  });
</script>

<svelte:body on:click={(ev) => closeWhenClickingOutside && onClick(ev)} />
<dialog class="dialog-box" bind:this={dialog} on:close={onClose}>
  <div class="dialog-contents" bind:this={contents}>
    <slot />
    {#if showCloseButton}
      <button class="close-button" on:click={onClose}>
        <CloseIcon class="close-icon" width={18} height={18} />
      </button>
    {/if}
  </div>
</dialog>

<style>
  .dialog-box {
    max-width: calc(100% - 40px);
    box-sizing: border-box;
    min-width: 200px;
    padding: 30px;
    background-color: #f7f7f7;
    border: none;
    border-radius: 8px;

    &::backdrop {
      background-color: #00000060;
    }
  }

  .dialog-contents {
    position: relative;
  }

  .close-button {
    position: absolute;
    width: 30px;
    height: 30px;
    top: -25px;
    right: -25px;
    border: none;
    cursor: pointer;
    border-radius: 20px;
    background: #00000000;
    color: rgb(77, 77, 77);
    transition: background 0.3s ease;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;

    &:enabled:hover {
      background: #00000015;
    }
  }
</style>