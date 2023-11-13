use tokio::sync::{mpsc, oneshot};

struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}
pub enum ActorMessage {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
    /// Shuts down the Actor and loses all of its connections.
    Shutdown,
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }

    async fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;

                // The `let _ =` ignores any errors when sending.
                //
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                let _ = respond_to.send(self.next_id);
            }
            ActorMessage::Shutdown => {
                self.receiver.close();
            }
        }
    }
}

async fn run_my_actor(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg).await;
    }
}

#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActorHandle {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = MyActor::new(receiver);
        tokio::spawn(run_my_actor(actor));
        Self { sender }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (respond_to, recv) = oneshot::channel();
        self.sender
            .send(ActorMessage::GetUniqueId { respond_to })
            .await
            .unwrap();
        recv.await.expect("Actor task has been killed")
    }

    /// Sending a cose messages to the Actor so it can close the reciver and shutdown propperly.
    pub async fn close(&self) -> Result<()> {
        self.sender.send(ActorMessage::Shutdown).await.unwrap();
        futures::join!(self.sender.closed());
        Ok(())
    }
}

use anyhow::Result;

// fn main() {
#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let handle = MyActorHandle::new();
    println!("Unique id: {}", handle.get_unique_id().await);
    println!("Unique id: {}", handle.get_unique_id().await);
    println!("Unique id: {}", handle.get_unique_id().await);
    println!("Unique id: {}", handle.get_unique_id().await);
    handle.close().await.unwrap();

    Ok(())
}
