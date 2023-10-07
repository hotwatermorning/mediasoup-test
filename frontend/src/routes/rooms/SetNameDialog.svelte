<script lang="ts">
  import { Button } from "~/component";
  import { getNameContext } from "~/context/NameProvider.svelte";
  import { countGrapheme } from "~/lib/text";
  const { nameStore } = getNameContext();

  const nameLengthLimit = 25;

  let name = "";
  $: count = countGrapheme(name);
  $: counterText = `現在：${count}文字/${nameLengthLimit}文字`;
  $: name = name.replaceAll("\n", "");

  const onSave = async () => {
    nameStore.set(name);
  };
</script>

<div class="layout">
  <div class="title">表示名を設定してください</div>
  <div class="layout-input">
    <input type="text" class="input" bind:value={name} />
    <div class={`counter ${count > nameLengthLimit ? "invalid" : ""}`}>
      {counterText}
    </div>
  </div>
  <div class="layout-controls">
    <Button
      on:click={onSave}
      primary={true}
      disabled={count === 0 || count >= nameLengthLimit}>設定して参加</Button
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

  .title {
    height: 34px;
    width: 100%;
    text-align: center;
    font-family: Noto Sans CJK JP;
    font-size: 18px;
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

  .layout-controls {
    display: flex;
    justify-content: center;
    gap: 20px;
  }
</style>
