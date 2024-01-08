# How to and NOT to consume a stream in NODE.js and

This project has 3 stream consumers, one good and one bad and the third is
batched.

All consumers consume a number from the stream and then sleep from a random
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

*Batched*

The Batched stream consumer is exactly like the good consumer except it reads
a more numbers from the stream and collects them into  batch of numbers to
process ate once before waiting for a random time. This mimics batching upp
a bunch of records and sending a single large update to the database in a single
query.

Initial testing indicates this is both faster and uses less application
resources than the bad method. Also it is less likely to crash becaus of an out
of memory error thou that depends on the size of the events in the stream.

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

