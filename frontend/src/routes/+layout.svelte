<script lang="ts">
  import NameProvider from "~/context/NameProvider.svelte";
  import ServerSettingProvider from "~/context/ServerSettingProvider.svelte";
</script>

<ServerSettingProvider>
  <NameProvider>
    <slot />
  </NameProvider>
</ServerSettingProvider>

<style global>
  @import url("https://fonts.googleapis.com/css2?family=Poiret+One&display=swap");

  :global(body) {
    background-repeat: no-repeat;
    background: radial-gradient(#11142c, #000211);
    background-color: #000211;
    margin: 0px;
    height: 100vh;
    width: 100vw;
    overflow-x: hidden;
    font-feature-settings: "clig" off, "liga" off;
    font-family: "Noto Sans JP", Roboto, "ヒラギノ角ゴシック Pro",
      "Hiragino Kaku Gothic Pro", Arial, メイリオ, Meiryo, Osaka,
      "ＭＳ Ｐゴシック", "MS PGothic", sans-serif;

    --col-primary: #00a2cb;
  }
</style>
