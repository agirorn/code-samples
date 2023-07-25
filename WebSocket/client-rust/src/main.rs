use anyhow::Result;

use futures_util::{SinkExt, StreamExt};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use url::Url;

async fn socket_to_server() -> Result<()> {
    let (mut socket, _) =
        connect_async(Url::parse("ws://localhost:9090/").expect("Can't connect to case count URL"))
            .await?;
    socket.send("home-ping".into()).await?;

    const TIMEOUT_START: u64 = 0;
    const TIMEOUT_MAX: u64 = 550;
    let mut timeout_ms: u64 = TIMEOUT_START;
    const TIMEOUT_MS_INC: u64 = 50;
    while let Some(msg) = socket.next().await {
        match msg {
            Ok(m) => {
                let text = m.into_text()?;
                if text == String::from("home-pong") {
                    let ms = timeout_ms;
                    timeout_ms += TIMEOUT_MS_INC;
                    if timeout_ms > TIMEOUT_MAX {
                        timeout_ms = TIMEOUT_START;
                    }
                    println!("text: {:?} sending bak ping in {}", text, ms);
                    sleep(Duration::from_millis(ms)).await;
                    socket.send("home-ping".into()).await?;
                } else {
                    println!("text: {:?}", text);
                }
            }
            Err(error) => println!("Error {}", error),
        }
    }
    socket.close(None).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = socket_to_server().await;
    Ok(())
}
