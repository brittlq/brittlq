import Axios from 'axios';
import { createStore } from 'vuex';
import { TOGGLE_CHAT_SIDEBAR } from './mutations';
import token from './get-token';

const axios = Axios.create({
  baseURL: process.env.VUE_APP_API_BASE,
});

export { axios };
export default createStore({
  state: {
    token: token,
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
  },
  actions: {},
  modules: {},
});
