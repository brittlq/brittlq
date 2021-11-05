import { createApp } from 'vue';
import { library } from '@fortawesome/fontawesome-svg-core';
import {
  faMinusCircle,
  faChevronDown,
  faChevronUp,
} from '@fortawesome/free-solid-svg-icons';
import { faTwitch } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import axios from 'axios';
import App from './App.vue';
import './assets/tailwind.css';
import store from './store';
import router from './router';

library.add(faMinusCircle, faTwitch, faChevronDown, faChevronUp);

createApp(App)
  .use(router)
  .use(store)
  .component('font-awesome-icon', FontAwesomeIcon)
  .component('fa-icon', FontAwesomeIcon)
  .use((app) => {
    app.config.globalProperties.$axios = axios;
  })
  .mount('#app');
