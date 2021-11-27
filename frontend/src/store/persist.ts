import { PiniaPluginContext } from 'pinia';

interface PersistPluginOptions {
  enabled: boolean;
  storage: 'localStorage' | 'sessionStorage';
  key?: string;
  reducer?: Function;
}

declare module 'pinia' {
  export interface DefineStoreOptionsBase<S, Store> {
    persist?: PersistPluginOptions;
  }
}

export function piniaPersistPlugin({ store, options }: PiniaPluginContext) {
  store.$subscribe((mutation, state) => {
    if (options.persist?.enabled) {
      const reduced = options.persist.reducer
        ? options.persist.reducer(state)
        : state;
      switch (options.persist.storage) {
        case 'localStorage':
          localStorage.setItem(
            options.persist.key ?? store.$id,
            JSON.stringify(reduced)
          );
          break;
        case 'sessionStorage':
        default:
          sessionStorage.setItem(
            options.persist.key ?? store.$id,
            JSON.stringify(reduced)
          );
      }
    }
  });
}
