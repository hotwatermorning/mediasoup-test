<script lang="ts">
  export let primary = false;
  export let disabled = false;
</script>

<button class={`button ${primary ? "primary" : ''}`} on:click {disabled}>
  <slot />
</button>

<style>
  .button {
    height: 40px;
    width: 180px;
    background-color: #ffffff;
    border: 1px solid var(--col-primary);
    border-radius: 3px;
    color: var(--col-primary);
    font-size: 15px;
    cursor: pointer;

    &:hover {
      filter: brightness(0.95);
    }

    &:disabled {
      cursor: default;
      border-color: #afafaf;
      color: #afafaf;

      &:hover {
        filter: none;
      }
    }

    &[class*='primary'] {
      background-color: var(--col-primary);
      color: #ffffff;

      &:hover {
        filter: brightness(1.1);
      }

      &:disabled {
        cursor: default;
        background-color: #afafaf;
        border-color: #afafaf;
        color: #ffffff;

        &:hover {
          filter: none;
        }
      }
    }
  }
</style>