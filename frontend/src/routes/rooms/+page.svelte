<script lang="ts">
  import { init, Participant, Participants } from "$lib/api.ts";
  import { Dialog, Header } from "~/component";
  import { getNameContext } from "~/context/NameProvider.svelte";
    import SetNameDialog from "./SetNameDialog.svelte";
    import { beforeNavigate } from "$app/navigation";
  const { nameStore } = getNameContext();

  let participants: Participants | undefined;

  beforeNavigate((navigation) => {
    if (participants?.getParticipants().length === 0) {
      return;
    }

    const yes = window.confirm('変更を保存せずにページを移動しますか？');
    if (!yes) {
      navigation.cancel();
    }
  });

  function onInit(node: HTMLVideoElement) {
    participants = new Participants(() => (participants = participants));
    node.onloadedmetadata = () => {
      node.play();
    };

    (async () => {
      await init($nameStore, participants, node);
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

  function bindParticipantElement(node: HTMLVideoElement, id: string) {
    if (!participants) {
      return;
    }

    node.onloadedmetadata = () => {
      node.play();
    };

    participants.bind(id, node);
  }
</script>

<main>
  <Header />
  <div id="container">
    {#if $nameStore === ""}
      <Dialog onClose={() => {}} showCloseButton={false} closeWhenClickingOutside={false}>
        <SetNameDialog />
      </Dialog>
    {:else}
    <div class="video-wrapper">
      <figure>
        <figcaption>You</figcaption>
        <video id="preview-send" muted controls use:onInit />
      </figure>
    </div>
    {#each participants?.getParticipants() ?? [] as { id, name }}
      <div class="video-wrapper">
        <figure>
          <figcaption>{name}</figcaption>
          <video muted controls use:bindParticipantElement={id} />
        </figure>
      </div>
    {/each}
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

  .video-wrapper {
    max-width: 1200px;
    width: 100%;
    height: auto;
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
