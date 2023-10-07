<script lang="ts">
  import { init as initApi, Participant, Participants } from "$lib/api.ts";

  let participants: Participants | undefined;
  let participantIds: string[] = [];

  function init(node: HTMLVideoElement) {
    participants = new Participants(() => participantIds = participants.getParticipantIds());
    node.onloadedmetadata = () =>
    {
      node.play();
    };

    (async () => {
       await initApi(participants, node);
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

  function bindParticipantElement(node: HTMLVideoElement, id: string)
  {
    if(!participants) {
      return;
    }

    node.onloadedmetadata = () =>
    {
      node.play();
    };

    participants.bind(id, node);
  }

</script>

<main>
  <div id="container">
    <figure>
        <video id="preview-send" muted controls use:init></video>
        <figcaption>You</figcaption>
    </figure>
    {#each participantIds as id}
    <figure>
      <video id="preview-send" muted controls use:bindParticipantElement={id}></video>
      <figcaption>Participant {id}</figcaption>
    </figure>
    {/each}
  </div>
</main>

<style>
</style>
