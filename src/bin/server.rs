use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use mini_redis::{Command, Frame};
use tokio::net::{TcpListener, TcpStream};

use learn_tokio::connection::Connection;

type StdMutex<T> = std::sync::Mutex<T>;
type Db = Arc<StdMutex<HashMap<String, Bytes>>>;

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection
        .read_frame()
        .await
        .expect("Failed to read content from socket")
    {
        let response = match Command::from_frame(frame) {
            Ok(command) => {
                tracing::info!("Start Execute command {:?}", command);
                let mut db = db.lock().expect("Failed to get db lock");
                let resp = match &command {
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
                };
                tracing::info!("Finish Execute command {:?}", command);
                resp
            }
            Err(error) => Frame::Error(format!("Failed to parse command: {:?}", error)),
        };
        connection
            .write_frame(&response)
            .await
            .expect("Failed to write frame");
    }
}

#[tokio::main]
async fn main() {
    // console_subscriber::init();
    learn_tokio::utils::config_logger();

    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Failed to bind localhost:6379");
    let db = Arc::new(StdMutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.expect("Failed to get socket");
        tracing::info!("Connect with {:?}", socket);

        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}
