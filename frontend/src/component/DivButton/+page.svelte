<script lang="ts">
  export let expand = false;
</script>

<button class="div-button" on:click on:dblclick style={expand ? `height: 100%; width: 100%;` : ''}>
  <slot />
</button>

<style>
  .div-button {
    position: relative;
    background: none;
    border: none;
    outline: none;
    appearance: none;
    padding: 0;
    cursor: pointer;
    text-align: left;
    color: inherit;
  }
</style>