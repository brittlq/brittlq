import { library } from '@fortawesome/fontawesome-svg-core';
import { faTwitch } from '@fortawesome/free-brands-svg-icons';
import { faTimesCircle } from '@fortawesome/free-regular-svg-icons';
import {
  faAngleDoubleLeft,
  faAngleDoubleRight,
  faChevronDown,
  faChevronUp,
  faMinusCircle,
} from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { AxiosStatic } from 'axios';
import { createApp } from 'vue';
import { Store } from 'vuex';
import App from './App.vue';
import './assets/tailwind.css';
import router from './router';
import store, { axios, State } from './store';
import { SET_TOKEN } from './store/twitch/operations';

declare module '@vue/runtime-core' {
  export interface ComponentCustomProperties {
    $axios: AxiosStatic;
    $store: Store<State>;
  }
}

library.add(
  faMinusCircle,
  faTwitch,
  faChevronDown,
  faChevronUp,
  faAngleDoubleRight,
  faAngleDoubleLeft,
  faTimesCircle
);

createApp(App)
  .use(router)
  .use(store)
  .component('fa-icon', FontAwesomeIcon)
  .use((app) => {
    app.config.globalProperties.$axios = axios;
  })
  .mount('#app');

// Dispatches an action in the store that checks for the hash fragment and sets the token if it exists
store.dispatch(SET_TOKEN);
