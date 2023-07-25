use std::{env, io::Error as IoError, net::SocketAddr};

use futures_util::{SinkExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut outgoing, mut incoming) = ws_stream.split();

    const TIMEOUT_START: u64 = 0;
    const TIMEOUT_MAX: u64 = 550;
    let mut timeout_ms: u64 = TIMEOUT_START;
    const TIMEOUT_MS_INC: u64 = 50;
    while let Some(msg) = incoming.next().await {
        match msg {
            Ok(m) => {
                let result = m.into_text();
                match result {
                    Ok(text) => {
                        if text == String::from("home-ping") {
                            let ms = timeout_ms;
                            timeout_ms += TIMEOUT_MS_INC;
                            if timeout_ms > TIMEOUT_MAX {
                                timeout_ms = TIMEOUT_START;
                            }
                            println!("text: {:?} sending bak ping in {}", text, ms);
                            sleep(Duration::from_millis(ms)).await;
                            let _ = outgoing.send("home-pong".into()).await;
                        } else {
                            println!("text: {:?}", text);
                        }
                    }
                    Err(e) => println!("text: {:?}", e),
                }
            }
            Err(error) => println!("Error {}", error),
        }
    }
    println!("{} disconnected", &addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9090".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }

    Ok(())
}
