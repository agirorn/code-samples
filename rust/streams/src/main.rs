#![allow(unused)]
use anyhow::Result;
use core::pin::Pin;
use futures::task::{Context, Poll};
use futures::Stream;
use futures_util::{pin_mut, stream::StreamExt};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug)]
struct Kline {
    pub value: i32,
}

impl Kline {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // let s = futures::stream::iter(vec![
    //     Kline::new(17),
    //     Kline::new(19),
    //     Kline::new(12),
    //     Kline::new(1),
    // ]);
    // useit(s).await?;

    // #########################################################
    // let mut c = CustomStrem {
    //     counter: 0,
    //     pend: 0,
    //     pend_max: 4,
    // };
    // // useit(c).await?;
    //
    // // if let Some(value) = input.next().await {
    // //     println!("got {:?}", value);
    // // }
    // print_form_strem(&mut c).await.unwrap();
    // print_form_strem(&mut c).await.unwrap();
    // print_form_strem(&mut c).await.unwrap();
    // print_form_strem(&mut c).await.unwrap();
    // print_form_strem(&mut c).await.unwrap();

    // #########################################################

    // let (tx, mut rx) = mpsc::channel(100);
    //
    // tokio::spawn(async move {
    //     tx.send("hello").await.unwrap();
    // });
    //
    // println!("{:?}", rx.recv().await);
    // println!("{:?}", rx.recv().await);

    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    let mut stream = ReceiverStream::new(rx);
    while let Some(value) = stream.next().await {
        println!("got {:?}", value);
    }

    Ok(())
}

async fn print_form_strem<S>(input: &mut S) -> Result<()>
where
    S: Stream<Item = Kline> + std::marker::Unpin,
{
    pin_mut!(input);
    if let Some(value) = input.next().await {
        println!("got {:?}", value);
    }
    Ok(())
}

async fn useit<S>(input: S) -> Result<()>
where
    S: Stream<Item = Kline>,
{
    pin_mut!(input);
    while let Some(value) = input.next().await {
        println!("got {:?}", value);
    }
    Ok(())
}

struct CustomStrem {
    pub counter: i32,
    pub pend: i32,
    pub pend_max: i32,
}

impl CustomStrem {
    pub fn next_counter(&mut self) -> i32 {
        self.counter += 1;
        self.counter
    }

    pub fn next_pend(&mut self) -> i32 {
        self.pend += 1;
        self.pend
    }
}

impl Stream for CustomStrem {
    type Item = Kline;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let instance = self.as_mut().get_mut();
        let value = instance.next_counter();
        if value <= 4 {
            return Poll::Ready(Some(Kline { value }));
        }
        // let instance = self.get_mut();
        if instance.next_pend() <= instance.pend_max {
            // Get a handle to the waker for the current task
            let waker = cx.waker().clone();
            // let when = Instant::now();
            let when: Instant = Instant::now() + Duration::from_millis(1000);
            // Spawn a timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            println!("Pending");
            return Poll::Pending;
            // Poll::Ready(None)
        }

        let value = self.get_mut().next_counter();
        Poll::Ready(Some(Kline { value }))
    }
}
