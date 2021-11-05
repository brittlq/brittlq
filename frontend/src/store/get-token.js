const { default: axios } = require('axios');

let token = null;

const hash_parameters = location.hash.substr(1);
//Since top level await is still experimental use the older IIFE technique to get async
(async () => {
  if (hash_parameters.length > 0) {
    const params = hash_parameters.split('&').reduce((res, item) => {
      var parts = item.split('=');
      res[parts[0]] = parts[1];
      return res;
    }, {});
    try {
      const response = await axios.post('/queue/token', params, {
        headers: { 'content-type': 'application/json' },
      });
      console.log(response);
    } catch (exc) {
      console.error(exc);
    }
  }
})();

export default token;
