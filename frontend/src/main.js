import { createApp } from 'vue';
import { library } from '@fortawesome/fontawesome-svg-core';
import { faMinusCircle } from '@fortawesome/free-solid-svg-icons';
import { faTwitch } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import axios from 'axios';
import App from './App.vue';
import './assets/tailwind.css';

library.add(faMinusCircle);
library.add(faTwitch);

const app = createApp(App) /*.use(router)*/
  .component('font-awesome-icon', FontAwesomeIcon)
  .mount('#app');

app.config.globalProperties.$axios = axios;
