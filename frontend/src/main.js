import { createApp } from 'vue';
import { library } from '@fortawesome/fontawesome-svg-core';
import { faMinusCircle } from '@fortawesome/free-solid-svg-icons';
import { faTwitch } from '@fortawesome/free-brands-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import App from './App.vue';
import './assets/tailwind.css';

library.add(faMinusCircle);
library.add(faTwitch);

createApp(App) /*.use(router)*/
  .component('font-awesome-icon', FontAwesomeIcon)
  .mount('#app');
