function log(...things: any[]) {
  things.forEach((thing) => console.log(thing));
}

function error(...things: any[]) {
  things.forEach((thing) => console.error(thing));
}

export default {
  log,
  error,
};
