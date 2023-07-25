/* eslint-disable no-console */
import WebSocket, { WebSocketServer } from 'ws';
import express from 'express';
import type { Express, Request, Response, NextFunction } from 'express';
import http from 'http';

const log = (msg: string): void => {
  // eslint-disable-next-line no-console
  console.log(msg);
};

const timeoutSTART = 0;
const timeoutMAX = 550;
let timeoutMs = timeoutSTART;
const timeoutMsInc = 50;

const run = (port: number): void => {
  const app: Express = express();

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  app.use((err: Error, req: Request, res: Response, _next: NextFunction) => {
    const message = err instanceof Error ? err.stack : err;
    res.status(500).send(message).end().flushHeaders();
  });

  const server = http.createServer(app);
  const wss = new WebSocketServer({ server });

  wss.on('connection', (ws) => {
    ws.on('error', console.error);

    ws.on('message', (data) => {
      if (data.toString() === 'home-ping') {
        const ms = timeoutMs;
        timeoutMs += timeoutMsInc;
        if (timeoutMs > timeoutMAX) {
          timeoutMs = timeoutSTART;
        }
        log(`received: ${data} on port ${port} sending home-pong in ${ms}`);

        setTimeout(() => {
          ws.send('home-pong');
        }, ms);
      } else {
        log(`received: ${data} on port ${port}`);
      }
    });

    ws.send(10);
  });

  server.listen(port, () => {
    log(`Server running on port: ${port}`);
  });
};

run(9090);

// #############################################################################

// import { createServer } from 'https';
// import { readFileSync } from 'fs';
// import { WebSocketServer } from 'ws';
//
// const server = createServer({});
// const wss = new WebSocketServer({ server });
//
// wss.on('connection', (ws) => {
//   ws.on('error', console.error);
//
//   ws.on('message', (data) => {
//     console.log('received: %s', data);
//   });
//
//   ws.send('something');
// });
//
// server.listen(8080);
