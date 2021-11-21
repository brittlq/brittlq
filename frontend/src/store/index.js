import { createStore } from 'vuex';
import VuexPersist from 'vuex-persist';
import { TOGGLE_CHAT_SIDEBAR, SET_TOKEN } from './mutations';
import axios from './axios';

const vuexLocal = new VuexPersist({
  storage: localStorage,
});

export { axios };
export default createStore({
  state: {
    token: null,
    botName: process.env.VUE_APP_BOT_NAME,
    channel: process.env.VUE_APP_TWITCH_CHANNEL,
    chatSidebarOpen: true, //TODO: this should come from application state, either stored in the backend or on the client
    oauth: {
      twitch: {
        clientId: process.env.VUE_APP_TWITCH_CLIENT_ID,
        redirectUri: process.env.VUE_APP_TWITCH_REDIRECT_URI,
        claims: '{"id_token":{"email":null,"email_verified":null }}',
        forceVerify: 'true',
        scope: 'chat:read chat:edit',
        responseType: 'token',
      },
    },
    obs: {
      address: 'ws://localhost:4444',
      password: '',
    },
  },
  getters: {
    twitchOauthUri: (state) => {
      const url = new URL('/oauth2/authorize', 'https://id.twitch.tv');
      url.searchParams.set('client_id', state.oauth.twitch.clientId);
      url.searchParams.set('redirect_uri', state.oauth.twitch.redirectUri);
      url.searchParams.set('response_type', state.oauth.twitch.responseType);
      url.searchParams.set('scope', state.oauth.twitch.scope);
      url.searchParams.set('force_verify', state.oauth.twitch.forceVerify);
      url.searchParams.set(
        'claims',
        encodeURIComponent(state.oauth.twitch.claims)
      );
      return url.toString();
    },
  },
  mutations: {
    [TOGGLE_CHAT_SIDEBAR](state) {
      state.chatSidebarOpen = !state.chatSidebarOpen;
    },
    [SET_TOKEN](state, token) {
      state.token = token;
    },
  },
  actions: {
    [SET_TOKEN]({ commit }) {
      // first set the token on state
      const hash_parameters = location.hash.substr(1);
      if (hash_parameters.length > 0) {
        const params = hash_parameters.split('&').reduce((res, item) => {
          var parts = item.split('=');
          res[parts[0]] = parts[1];
          return res;
        }, {});
        const token = params['access_token'];
        // optimistically set the token on the state before sending to the backend
        commit(SET_TOKEN, token);
        //Since top level await is still experimental use the older IIFE technique to get async
        (async () => {
          try {
            const response = await axios.post('/queue/token', params, {
              headers: { 'content-type': 'application/json' },
            });
            console.log(response);
          } catch (exc) {
            console.error(exc);
          }
        })();
      }
    },
  },
  modules: {},
  plugins: [vuexLocal.plugin],
});
