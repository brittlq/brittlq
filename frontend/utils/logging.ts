function log(...things: any[]): void {
  things.forEach((thing) => console.log(thing));
}

function error(...things: any[]): void {
  things.forEach((thing) => console.error(thing));
}

export default {
  log,
  error,
};
