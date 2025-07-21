use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use mini_redis::{Command, Frame};
use tokio::net::{TcpListener, TcpStream};

use learn_tokio::connection::Connection;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection
        .read_frame()
        .await
        .expect("Failed to read content from socket")
    {
        let response = Command::from_frame(frame).map_or_else(
            |error| Frame::Error(format!("Failed to parse command: {:?}", error)),
            |command| {
                let mut db = db.lock().expect("Failed to get db");
                tracing::info!("Execute command {:?}", command);
                match command {
                    Command::Get(cmd) => {
                        if let Some(value) = db.get(cmd.key()) {
                            Frame::Bulk(value.clone())
                        } else {
                            Frame::Null
                        }
                    }
                    Command::Set(cmd) => {
                        db.insert(cmd.key().to_string(), cmd.value().clone());
                        Frame::Simple("OK".to_string())
                    }
                    cmd => Frame::Error(format!("unimplemented: {:?}", cmd)),
                }
            },
        );

        connection
            .write_frame(&response)
            .await
            .expect("Failed to write frame");
    }
}
#[tokio::main]
async fn main() {
    // console_subscriber::init();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Failed to bind localhost:6379");
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.expect("Failed to get socket");
        tracing::info!("Connect with {:?}", socket);
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
