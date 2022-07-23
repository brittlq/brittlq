import logging from '@/utils/logging';
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
      hydrater?: (
        storedState: string,
        context: PiniaPluginContext<string, S>
      ) => Partial<S>;
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

export function piniaPersistPlugin(context: PiniaPluginContext) {
  const { options, store } = context;
  if (options.persist?.enabled) {
    const persister = persistMethod(
      options.persist.storage,
      options.persist.persister
    );
    const storageKey = options.persist?.key ?? store.$id;
    // subscribe to state mutations to set the desired state into the given storage medium
    const storedState = persister.getItem(storageKey);
    if (storedState) {
      if (options.persist?.hydrater) {
        options.persist.hydrater(storedState, context);
      } else {
        store.$patch(JSON.parse(storedState));
      }
    }
    store.$subscribe((mutation, state) => {
      const reduced = options.persist?.reducer
        ? options.persist.reducer(state)
        : state;
      persister.setItem(storageKey, JSON.stringify(reduced));
    });
  }
}
