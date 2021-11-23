export default {
  state: {
    queue: [],
    partySize: 4,
    partyTime: 5 * 60,
    disabled: false,
    open: false,
  },
  mutations: {
    [SET_TIME]() {},
    [SET_PARTY_SIZE]() {},
    [TOGGLE_OPEN]() {},
    [SET_DISABLED]() {},
  },
  actions: {
    [UPDATE]() {},
  },
};
