<script lang="ts">
  import { Dialog, Header } from "~/component";
  import { getClientSettingContext } from "~/context/ClientSettingProvider.svelte";
  import SetNameDialog from "./JoinSettingDialog.svelte";
  import VideoList from "./VideoList.svelte";
  const { clientSettingStore, isValidSetting } = getClientSettingContext();
</script>

<main>
  <Header />
  <div id="container">
    {#if isValidSetting($clientSettingStore) === false}
      <Dialog
        onClose={() => {}}
        showCloseButton={false}
        closeWhenClickingOutside={false}
        closeWhenEscapeKeyPressed={false}
      >
        <SetNameDialog />
      </Dialog>
    {:else}
      <VideoList />
    {/if}
  </div>
</main>

<style>
  main {
    min-width: 500px;
  }

  #container {
    padding: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>
