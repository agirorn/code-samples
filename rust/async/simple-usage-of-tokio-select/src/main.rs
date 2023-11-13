// fn main() {
//     println!("Hello, world!");
// }
use rand::rngs::OsRng;
use rand::Rng;
use tokio::select;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() {
    // Create an MPSC channel
    let (tx, mut rx) = mpsc::channel::<String>(32); // 32 is the buffer size

    // Spawn a task to send messages to the channel
    tokio::spawn(async move {
        let mut rng = OsRng;
        let max_delay = 5;
        let mut wait: u64 = rng.gen_range(1..=max_delay);
        tx.send(format!("First message in {}", wait)).await.unwrap();
        for i in 0..5 {
            tokio::time::sleep(Duration::from_secs(wait)).await;
            wait = rng.gen_range(1..=max_delay);
            tx.send(format!("Message {}, next in {}", i, wait))
                .await
                .unwrap();
        }
    });

    // Use tokio::select! to wait for messages from the channel or the fixed interval
    let interval_sec = 1;
    let mut interval = interval(Duration::from_secs(interval_sec));

    loop {
        select! {
            // Receive messages from the channel
            msg = rx.recv() => {
                match msg {
                    Some(message) => {
                        println!("Received: {}", message);
                    }
                    None => {
                        println!("RX Channel closed, ending loop");
                        // break here ends the loop terminating preventing select! form checking
                        // any other branch and as thus ending the application
                        break;
                    }
                }
            }

            // Perform an action at a fixed interval
            _ = interval.tick() => {
                println!("Doing something at every {} sec interval", interval_sec);
            }
        }
    }
}
