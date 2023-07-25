import WebSocket from 'ws';

interface Options {
  host: string;
  path?: string;
}

const log = (msg: string): void => {
  // eslint-disable-next-line no-console
  console.log(msg);
};

const timeoutSTART = 0;
const timeoutMAX = 550;
let timeoutMs = timeoutSTART;
const timeoutMsInc = 25;

const run = ({ host, path = '/' }: Options): void => {
  const ws = new WebSocket(`ws://${host}${path}`);

  ws.on('message', (message) => {
    const msg = message.toString();
    if (msg === 'home-pong') {
      log(`Got ${msg} sending home-ping in ${timeoutMs} on host ${host}`);
      setTimeout(() => {
        ws.send('home-ping');
        timeoutMs += timeoutMsInc;
        if (timeoutMs > timeoutMAX) {
          timeoutMs = timeoutSTART;
        }
      }, timeoutMs);
    } else {
      log(`Got ${msg}`);
    }
  });
  ws.once('open', () => {
    ws.send('home-ping');
  });
};

run({
  host: '127.0.0.1:9090',
  // path: '/test',
});
