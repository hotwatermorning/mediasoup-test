<script lang="ts" context="module">
  import { getContext, onMount, setContext } from "svelte";
  import { get, writable, type Writable } from "svelte/store";

  const store = writable<string>("");

  const createContext = () => {
    return {
      nameStore: store,
    };
  };

  const KEY = "name";
  export type NameContext = ReturnType<typeof createContext>;
  export const getNameContext = () => getContext<NameContext>(KEY);
</script>

<script lang="ts">
	setContext<NameContext>(KEY, createContext());
</script>

<slot />
