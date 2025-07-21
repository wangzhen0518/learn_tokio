use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

async fn client_server(mut rx: mpsc::Receiver<Command>) {
    let mut client = mini_redis::client::connect("127.0.0.1:6379")
        .await
        .expect("Failed to connect to 127.0.0.1:8379");

    while let Some(command) = rx.recv().await {
        match command {
            Command::Get { key, resp } => {
                let _ = resp.send(client.get(&key).await);
            }
            Command::Set { key, val, resp } => {
                let _ = resp.send(client.set(&key, val).await);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);
    let manager = tokio::spawn(client_server(rx));

    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
        let resp = resp_rx.await.expect("Failed to receive response");
        println!("{:?}", resp);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };
        tx2.send(cmd).await.unwrap();
        let resp = resp_rx.await.expect("Failed to receive response");
        if resp.is_ok() {
            println!("Set success");
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
