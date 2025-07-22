#![allow(unused_imports)]

use std::sync::Arc;

use bytes::Bytes;
use tokio::sync::{Mutex, mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
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
    // let client = Arc::new(Mutex::new(client));

    while let Some(command) = rx.recv().await {
        tracing::info!("Send command: {:?}", command);
        match command {
            Command::Get { key, resp } => {
                // let client = client.clone();
                // tokio::spawn(async move {
                //     let res = client.lock().await.get(&key).await;
                let res = client.get(&key).await;
                tracing::info!("Command GET receive: {:?}", res);
                let _ = resp.send(res);
                // });
            }
            Command::Set { key, val, resp } => {
                // let client = client.clone();
                // tokio::spawn(async move {
                // let res = client.lock().await.set(&key, val).await;
                let res = client.set(&key, val).await;
                tracing::info!("Command SEND receive: {:?}", res);
                let _ = resp.send(res);
                // });
            }
        }
    }
}

#[tokio::main]
async fn main() {
    learn_tokio::utils::config_logger();

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
