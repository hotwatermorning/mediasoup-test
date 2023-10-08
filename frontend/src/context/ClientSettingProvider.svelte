<script lang="ts" context="module">
  import { getContext, onMount, setContext } from "svelte";
  import { get, writable, type Writable } from "svelte/store";

  export type ClientSetting = {
    name: string;
    cameraId: string;
    micId: string;
  };

  const createDefault = (): ClientSetting => {
    return {
      name: "",
      cameraId: "",
      micId: "",
    };
  };

  const store = writable<ClientSetting>(createDefault());

  const createContext = () => {
    return {
      clientSettingStore: store,
      isValidSetting: (store: ClientSetting) => {
        return store.name.trim() !== "";
      },
      setName: (name: string) => {
        return store.update((cur) => {
          const newValue = structuredClone(cur);
          newValue.name = name;
          return newValue;
        });
      },
      setCameraId: (cameraId: string) => {
        return store.update((cur) => {
          const newValue = structuredClone(cur);
          newValue.cameraId = cameraId;
          return newValue;
        });
      },
      setMicId: (micId: string) => {
        return store.update((cur) => {
          const newValue = structuredClone(cur);
          newValue.micId = micId;
          return newValue;
        });
      },
    };
  };

  const KEY = "client-setting";
  type Context = ReturnType<typeof createContext>;
  export const getClientSettingContext = () => getContext<Context>(KEY);
</script>

<script lang="ts">
	setContext<Context>(KEY, createContext());
</script>

<slot />
