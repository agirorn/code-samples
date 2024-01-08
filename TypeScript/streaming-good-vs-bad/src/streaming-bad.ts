import { Readable } from 'stream';
import { SEC, RAND_MAX, sleep , getRandomInt, consoleBanner  } from './common';

const printBanner = () => {
  const message = `
  Consuming a stream of number as fast as the stream can emit them on the
  on('data') event handler and then waiting for a random ${RAND_MAX} ms before
  printing to the terminal that the work is compleate.
  This is imitating database call latency.
`.split("\n").map((l) => l.trim()).join(" ");

  consoleBanner(message);
}

const main = async () => {
  printBanner();
  await sleep(1 * SEC);
  let num = 0;
  const stream = new Readable({
    objectMode: true,
    read() {
      const val = num++;
      console.log("Poducing", val);
      this.push(val);
    }
  });

  stream.on('data', async (val) => {

    const ms =  getRandomInt(RAND_MAX)
    console.log(`Consuming: ${val} -- sleeping for ${ms}`);
    await sleep(getRandomInt(ms));
    console.log(`CONSUMED: ${val} after sleeping for ${ms}`);
  });
}

main().then(
  console.log,
  console.error,
);

