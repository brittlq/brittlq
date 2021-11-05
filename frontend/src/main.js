import { createApp } from 'vue';
import { library } from '@fortawesome/fontawesome-svg-core';
import { faMinusCircle } from '@fortawesome/free-solid-svg-icons';
import { faTwitch } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import axios from 'axios';
import App from './App.vue';
import './assets/tailwind.css';
import store from './store';

library.add(faMinusCircle);
library.add(faTwitch);

createApp(App) /*.use(router)*/
  .use(store)
  .component('font-awesome-icon', FontAwesomeIcon)
  .use((app) => {
    app.config.globalProperties.$axios = axios;

    app.config.globalProperties.settings = {
      oauth: {
        twitch: {
          clientId: process.env.VUE_APP_TWITCH_CLIENT_ID,
          redirectUri: process.env.VUE_APP_TWITCH_REDIRECT_URI,
        },
      },
    };
  })
  .mount('#app');
