import { PiniaPluginContext } from 'pinia';

type StorageName = 'localStorage' | 'sessionStorage' | 'custom';
type Persister = Pick<Storage, 'getItem' | 'setItem'>;
declare module 'pinia' {
  export interface DefineStoreOptionsBase<S, Store> {
    persist?: {
      enabled: boolean;
      storage?: StorageName;
      key?: string;
      reducer?: (state: S) => Partial<S>;
      persister?: Persister;
    };
  }
}

function persistMethod(
  storageType: StorageName = 'sessionStorage',
  persister: Persister | undefined
): Persister {
  switch (storageType) {
    case 'localStorage':
      return localStorage;
    case 'custom':
      if (!persister) {
        throw new Error(
          'When using the custom persister option, the persist.persister object must be set'
        );
      }
      return persister;
    case 'sessionStorage':
    default:
      return sessionStorage;
  }
}

export function piniaPersistPlugin({ store, options }: PiniaPluginContext) {
  if (options.persist?.enabled) {
    const persister = persistMethod(
      options.persist.storage,
      options.persist.persister
    );
    const storageKey = options.persist?.key ?? store.$id;
    const storedState = persister.getItem(storageKey);
    if (storedState) {
      // TODO: handle the case where the custom persister returns a hydrated object
      store.$patch(JSON.parse(storedState));
    }
    // subscribe to state mutations to set the desired state into the given storage medium
    store.$subscribe((mutation, state) => {
      const reduced = options.persist?.reducer
        ? options.persist.reducer(state)
        : state;
      persister.setItem(storageKey, JSON.stringify(reduced));
    });
  }
}
