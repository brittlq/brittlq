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
import { createPinia } from 'pinia';
import App from './App.vue';
import './assets/tailwind.css';
import router from './router';
import axios from './store/axios';
import { useTwitchStore } from './store/twitch';

declare module '@vue/runtime-core' {
  export interface ComponentCustomProperties {
    $axios: AxiosStatic;
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

const pinia = createPinia();

createApp(App)
  .use(router)
  .use(pinia)
  .component('fa-icon', FontAwesomeIcon)
  .use((app) => {
    app.config.globalProperties.$axios = axios;
  })
  .mount('#app');

// set the twitch token if it exists
useTwitchStore().setToken();
