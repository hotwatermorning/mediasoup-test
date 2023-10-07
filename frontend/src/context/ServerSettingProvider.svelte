<script lang="ts" context="module">
  import { getContext, onMount, setContext } from "svelte";
  import { get, writable, type Writable } from "svelte/store";

  type ServerSetting = {
    turnServerUrl: string;
    username: string;
    password: string;
  }

  const createDefaultServerSetting = () => {
    return {
      turnServerUrl: "",
      username: "",
      password: ""
    };
  }
  const serverSettingStore = writable<ServerSetting>(createDefaultServerSetting());

  const createContext = () => {
    return {
      serverSettingStore,
      setTurnServerUrl: (url: string) => {
        serverSettingStore.update((cur) => {
          return {
            ...cur, turnServerUrl: url
          };
        });
      },
      setUsername: (username: string) => {
        serverSettingStore.update((cur) => {
          return {
            ...cur, username
          };
        });
      },
      setPassword: (password: string) => {
        serverSettingStore.update((cur) => {
          return {
            ...cur, password
          };
        });
      },
    };
  };

  const KEY = "server-setting";
  export type ServerSettingContext = ReturnType<typeof createContext>;
  export const getServerSettingContextContext = () => getContext<ServerSettingContext>(KEY);
</script>

<script lang="ts">
	setContext<ServerSettingContext>(KEY, createContext());
</script>

<slot />
