import { writable } from 'svelte/store';

export const createLoadingGuard = <T = void>(initialLoadingState = false) => {
  const loadingStore = writable<boolean>(initialLoadingState);

  const loadingGuard = async (proc: () => Promise<T>): Promise<T> => {
    loadingStore.set(true);
    try {
      const result = await proc();
      loadingStore.set(false);

      return result;
    } catch(e) {
      loadingStore.set(false);
      throw e;
    }
  };

  return { loadingStore, loadingGuard };
};