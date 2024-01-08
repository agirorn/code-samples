# How to and NOT to consume a stream in NODE.js and

This project has 2 stream consumers, one good and the other bad.

Both consumers consume a number from the stream and then sleep from a random
time before printing to the terminal that they have completed the work.

The good consumer consumes a single number from the stream at a time and sleep
for a random time and repeats the process.

The difference is that the good one processes a single number at a time by
treating the stream as an async iterators. This allows the stream to maintain
the back pressure and does not overwhelm the nodejs process.

The bad one on the other hand registers a function on the `on("data")` event
handler and tries to process every call to on data. This seams to be much faster
at first bug eventually consumes too much memory and nodejs terminates the
process with the error:

> "FATAL ERROR: Reached heap limit Allocation failed - JavaScript heap out of memory"

If the bad approach would be used in real life application that would be reading
from one stream and writing it to a database from instance. It whould probably
drown the database server in query's and make the thing much worse than it needs
to be.

## Setup

```
yarn install
yarn build
```

## Usage

To start the good streamer run this

```
  yarn start:good
```

To start the bad streamer run this

```
  yarn start:bad
```

