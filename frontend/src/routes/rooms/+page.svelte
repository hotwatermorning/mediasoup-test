<script lang="ts">
  import { Dialog, Header } from "~/component";
  import { getNameContext } from "~/context/NameProvider.svelte";
  import SetNameDialog from "./SetNameDialog.svelte";
  import { beforeNavigate } from "$app/navigation";
  import VideoList from "./VideoList.svelte";
  const { nameStore } = getNameContext();
</script>

<!--
  # レイアウトの方針

  平方数のときは、その通りに並べる。
  それ以外のときはそれより小さい
-->
<main>
  <Header />
  <div id="container">
    {#if $nameStore === ""}
      <Dialog
        onClose={() => {}}
        showCloseButton={false}
        closeWhenClickingOutside={false}
      >
        <SetNameDialog />
      </Dialog>
    {:else}
      <VideoList name={$nameStore} />
    {/if}
  </div>
</main>

<style>
  #container {
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
