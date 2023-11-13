use anyhow::Result;
use tokio::sync::{mpsc, oneshot};

struct Actor {
    receiver: mpsc::Receiver<Message>,
    next_id: u32,
}
pub enum Message {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
    /// Shuts down the Actor and loses all of its connections.
    Shutdown,
}

impl Actor {
    fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Actor {
            receiver,
            next_id: 0,
        }
    }

    async fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::GetUniqueId { respond_to } => {
                self.next_id += 1;

                // The `let _ =` ignores any errors when sending.
                //
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                let _ = respond_to.send(self.next_id);
            }
            Message::Shutdown => {
                self.receiver.close();
            }
        }
    }
}

async fn run_my_actor(mut actor: Actor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}

#[derive(Clone)]
pub struct Handle {
    sender: mpsc::Sender<Message>,
}

impl Handle {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Actor::new(receiver);
        tokio::spawn(run_my_actor(actor));
        Self { sender }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (respond_to, recv) = oneshot::channel();
        self.sender
            .send(Message::GetUniqueId { respond_to })
            .await
            .unwrap();
        recv.await.expect("Actor task has been killed")
    }

    /// Sending a cose messages to the Actor so it can close the reciver and shutdown propperly.
    pub async fn close(&self) -> Result<()> {
        self.sender.send(Message::Shutdown).await.unwrap();
        futures::join!(self.sender.closed());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let actor = Handle::new();
    println!("Unique id: {}", actor.get_unique_id().await);
    println!("Unique id: {}", actor.get_unique_id().await);
    println!("Unique id: {}", actor.get_unique_id().await);
    println!("Unique id: {}", actor.get_unique_id().await);
    actor.close().await.unwrap();

    Ok(())
}
