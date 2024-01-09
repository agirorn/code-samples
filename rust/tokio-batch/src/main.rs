#![allow(unused)]
use anyhow::Result;
use std::io::{self, Write};

use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, timeout, Instant, Sleep};

use rand::Rng;
use std::time::Duration;
use tokio::select;

#[tokio::main]
async fn main() -> Result<()> {
    let (sender, mut receiver) = mpsc::channel(10);
    tokio::spawn(the_sender(sender));
    let mut batch = Batch::new(8, 5000);

    loop {
        select! {
            result = batch.timeout() => {
                batch.commit().await;
            },
            msg = receiver.recv() => {
                match msg {
                    Some(value) => {
                        batch.save(value).await;
                    }
                    None => {
                        println!(" the party is over");
                        return Ok(())
                    }
                }
            }
        }
    }

    Ok(())
}

struct Batch<T> {
    start: Instant,
    size: usize,
    max_collection_time_in_mills: u64,
    numbers: Vec<T>,
}

impl<T> Batch<T>
where
    T: std::fmt::Debug,
{
    pub fn new(size: usize, max_collection_time_in_mills: u64) -> Self {
        Self {
            start: Instant::now(),
            size,
            max_collection_time_in_mills,
            numbers: vec![],
        }
    }

    fn timedout(&self) -> bool {
        self.start.elapsed().as_millis() as u64 > self.max_collection_time_in_mills
    }

    pub async fn save(&mut self, value: T) -> Result<()> {
        self.numbers.push(value);
        if self.timedout() || self.numbers.len() >= self.size {
            self.commit().await;
        }
        Ok(())
    }

    pub async fn commit(&mut self) -> Result<()> {
        println!(
            "Saving batch values({}): {:#?}",
            self.numbers.len(),
            self.numbers
        );
        // println!("Saving batch values: {:#?}", self.numbers);
        self.numbers = vec![];
        self.start = Instant::now();
        Ok(())
    }

    pub fn timeout(&self) -> Sleep {
        let elapsed = self.start.elapsed().as_millis() as u64;
        let timeout_left = if elapsed > self.max_collection_time_in_mills {
            0
        } else {
            self.max_collection_time_in_mills - elapsed
        };
        sleep(Duration::from_millis(timeout_left))
    }
}

async fn the_sender(mut sender: mpsc::Sender<i64>) {
    let mut i = 0;
    loop {
        i += 1;
        println!("sending: {}", i);
        sender.send(i).await;
        let random = {
            let mut rng = rand::thread_rng();
            rng.gen_range(50..1500)
        };
        sleep(Duration::from_millis(random)).await;
    }
}
