import { Readable } from 'stream';
import { SEC, RAND_MAX, sleep , getRandomInt, consoleBanner  } from './common';

const printBanner = () => {
  const message = `
  Consuming a stream of number one number at a time and waiting for a
  random ${RAND_MAX} ms before printing to the terminal that the work is
  compleate and then reading the next number from the stream imitading
  database query latency.
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

  for await (const val of stream) {
    const ms =  getRandomInt(RAND_MAX)
    console.log(`Consuming: ${val} -- sleeping for ${ms}`);
    await sleep(getRandomInt(ms));
  }
}

main().then(
  console.log,
  console.error,
);

