import { ActionContext } from 'vuex';
import { axios, State as RootState } from '..';
import { SET_TOKEN } from './operations';
import chat, { State as ChatState } from './chat';

export interface State {
  clientId: string;
  redirectUri: string;
  claims: string;
  forceVerify: string;
  scope: string;
  responseType: string;
  token: string | undefined;
  chat?: ChatState;
}

export default {
  modules: { chat },
  state: (): State => ({
    clientId: process.env.VUE_APP_TWITCH_CLIENT_ID ?? '',
    redirectUri: process.env.VUE_APP_TWITCH_REDIRECT_URI ?? '',
    claims: '{"id_token":{"email":null,"email_verified":null }}',
    forceVerify: 'true',
    scope: 'chat:read chat:edit',
    responseType: 'token',
    token: undefined,
  }),
  mutations: {
    [SET_TOKEN](state: State, { token }: { token: string }) {
      state.token = token;
    },
  },
  getters: {
    twitchOauthUri(state: State): string {
      const url = new URL('/oauth2/authorize', 'https://id.twitch.tv');
      url.searchParams.set('client_id', state.clientId);
      url.searchParams.set('redirect_uri', state.redirectUri);
      url.searchParams.set('response_type', state.responseType);
      url.searchParams.set('scope', state.scope);
      url.searchParams.set('force_verify', state.forceVerify);
      url.searchParams.set('claims', encodeURIComponent(state.claims));
      return url.toString();
    },
    hasToken(state: State): boolean {
      return !!state.token;
    },
  },
  actions: {
    [SET_TOKEN]({ commit }: ActionContext<State, RootState>) {
      // first set the token on state
      const hash_parameters = location.hash.substr(1);
      if (hash_parameters.length > 0) {
        const params = hash_parameters
          .split('&')
          .reduce<Record<string, string>>((res, item) => {
            const parts = item.split('=');
            res[parts[0]] = parts[1];
            return res;
          }, {});
        const token: string = params['access_token'];
        // optimistically set the token on the state before sending to the backend
        commit(SET_TOKEN, { token });
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
};
