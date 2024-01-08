export const SEC = 1000;
export const RAND_MAX = 1000;
export const BATCH_SIZE = 5000;
export const BATCH_MAX_AGE_MS = 1000;
export const sleep = async (ms: number) => new Promise((resolve)=> {
  setTimeout(resolve, ms);
});

export const getRandomInt = (max: number) => Math.floor(Math.random() * max);

export const line = () => {
  console.log('-'.repeat(process.stdout.columns));
};

export const consoleBanner = (message: string) => {
  line();
  console.log(message);
  line();
}

