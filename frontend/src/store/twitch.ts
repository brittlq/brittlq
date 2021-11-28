import { defineStore } from 'pinia';
import axios from './axios';
import logging from '../utils/logging';
import { UndefinedString } from '.';

export const useTwitchStore = defineStore('twitch', {
  state: () => ({
    clientId: process.env.VUE_APP_TWITCH_CLIENT_ID as UndefinedString,
    redirectUri: process.env.VUE_APP_TWITCH_REDIRECT_URI as UndefinedString,
    claims: '{"id_token":{"email":null,"email_verified":null }}',
    forceVerify: 'true',
    scope: 'chat:read chat:edit',
    responseType: 'token',
    token: undefined as UndefinedString,
  }),
  getters: {
    twitchOauthUri(state): string {
      const url = new URL('/oauth2/authorize', 'https://id.twitch.tv');
      url.searchParams.set('client_id', state.clientId!);
      url.searchParams.set('redirect_uri', state.redirectUri!);
      url.searchParams.set('response_type', state.responseType);
      url.searchParams.set('scope', state.scope);
      url.searchParams.set('force_verify', state.forceVerify);
      url.searchParams.set('claims', encodeURIComponent(state.claims));
      return url.toString();
    },
    hasToken(state): boolean {
      return !!state.token;
    },
  },
  actions: {
    async setToken() {
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
        this.token = token;
        //Since top level await is still experimental use the older IIFE technique to get async
        try {
          const response = await axios.post('/queue/token', params, {
            headers: { 'content-type': 'application/json' },
          });
          logging.log(response);
        } catch (exc) {
          logging.error(exc);
        }
      }
    },
  },
  persist: {
    enabled: true,
    reducer(state) {
      return {
        token: state.token,
      };
    },
  },
});
