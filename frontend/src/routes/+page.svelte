<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "~/component";

  let roomId = "";

  const onJoinNewRoom = () => {
    goto(`/rooms`);
  };

  const onJoinTargetRoom = () => {
    goto(`/rooms?roomId=${roomId}`);
  };
</script>

<main>
  <div class="layout">
    <h1>
      <div class="line1">
        m<span class="small gradient">e</span>dia<span class="gradient"
          >soup</span
        >
      </div>
      <div class="padding">{" "}</div>
      <div class="line2">video chat test</div>
    </h1>
    <div class="control-section">
      <p>新しいルームを作成して入室する</p>
      <Button on:click={onJoinNewRoom} primary={true} >{"入室"}</Button>
    </div>
    <div class="control-section">
      <p>既存のルームに入室する</p>
      <input type="text" bind:value={roomId} placeholder="ルームID" />
      <Button on:click={onJoinTargetRoom} primary={true} >{"入室"}</Button>
    </div>
  </div>
</main>

<style>
  .layout {
    color: white;
    padding: 40px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  h1 {
    font-family: "Poiret One";
    letter-spacing: -0.04em;
    font-size: 3.5rem;
    font-weight: 200;
    color: #ccc;

    display: flex;

    @media screen and (max-width: css-breakpoint-sp) {
      flex-direction: column;
    }
  }

  .padding {
    white-space: pre;
    @media screen and (max-width: css-breakpoint-sp) {
      display: none;
    }
  }

  .small {
    font-size: 3rem;
  }

  .gradient {
    background-image: linear-gradient(0deg, #1eb6e8 10%, #9335db 100%);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-size: 100%;
  }

  .control-section {
    display: flex;
    flex-direction: column;
    gap: 30px;
  }

  p {
    margin-block: unset;
  }

  input {
    padding: 10px;
    color: #444;
    font-size: 1.2rem;
    display: block;
    max-width: 400px;
    width: inherit;
  }
</style>
