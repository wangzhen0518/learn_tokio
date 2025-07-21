use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[allow(unused)]
async fn io_copy(mut socket: TcpStream) {
    let (mut reader, mut writer) = socket.split();
    if io::copy(&mut reader, &mut writer).await.is_err() {
        eprintln!("Failed to copy");
    };
}

#[allow(unused)]
async fn manual_copy(mut socket: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => return,
            Ok(n) => {
                if socket.write_all(&buffer[..n]).await.is_err() {
                    return;
                }
            }
            Err(_) => return,
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6142")
        .await
        .expect("Failed to bind to 127.0.0.1:6142");
    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Listener failed to get connect");

        // tokio::spawn(io_copy(socket));
        tokio::spawn(manual_copy(socket));
    }
}
