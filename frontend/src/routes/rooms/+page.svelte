<script lang="ts">
  import { Dialog, Header } from "~/component";
  import { getClientSettingContext } from "~/context/ClientSettingProvider.svelte";
  import SetNameDialog from "./JoinSettingDialog.svelte";
  import { beforeNavigate } from "$app/navigation";
  import VideoList from "./VideoList.svelte";
  const { clientSettingStore, isValidSetting } = getClientSettingContext();
</script>

<!--
  # レイアウトの方針

  平方数のときは、その通りに並べる。
  それ以外のときはそれより小さい
-->
<main>
  <Header />
  <div id="container">
    {#if isValidSetting($clientSettingStore) === false}
      <Dialog
        onClose={() => {}}
        showCloseButton={false}
        closeWhenClickingOutside={false}
      >
        <SetNameDialog />
      </Dialog>
    {:else}
      <VideoList />
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
