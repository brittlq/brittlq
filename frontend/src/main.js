import { createApp } from 'vue';
import { library } from '@fortawesome/fontawesome-svg-core';
import {
  faMinusCircle,
  faChevronDown,
  faChevronUp,
  faAngleDoubleLeft,
  faAngleDoubleRight,
} from '@fortawesome/free-solid-svg-icons';
import { faTimesCircle } from '@fortawesome/free-regular-svg-icons';
import { faTwitch } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import App from './App.vue';
import './assets/tailwind.css';
import store, { axios } from './store';
import router from './router';
import { SET_TOKEN } from './store/mutations';

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

store.dispatch(SET_TOKEN);
