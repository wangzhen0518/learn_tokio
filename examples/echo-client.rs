use std::sync::Arc;

use bytes::Bytes;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::Notify;
use tokio::sync::{Mutex, mpsc};

#[allow(unused)]
struct AutoShutdownWriteHalf {
    inner: Arc<Mutex<WriteHalf<TcpStream>>>,
    shutdown_trigger: Arc<Notify>,
}

impl AutoShutdownWriteHalf {
    #[allow(unused)]
    fn new(inner: WriteHalf<TcpStream>) -> Self {
        let inner = Arc::new(Mutex::new(inner));
        let shutdown_trigger = Arc::new(Notify::new());

        let inner_clone = inner.clone();
        let shutdown_trigger_clone = shutdown_trigger.clone();

        tokio::spawn(async move {
            shutdown_trigger_clone.notified().await;
            let _ = inner_clone.lock().await.shutdown().await;
        });

        Self {
            inner,
            shutdown_trigger,
        }
    }

    #[allow(unused)]
    async fn write_all(&mut self, src: &[u8]) -> io::Result<()> {
        self.inner.lock().await.write_all(src).await
    }
}

impl Drop for AutoShutdownWriteHalf {
    fn drop(&mut self) {
        self.shutdown_trigger.notify_one();
    }
}

struct AutoShutdownWriteHalfV2 {
    sender: mpsc::Sender<Bytes>,
}

impl AutoShutdownWriteHalfV2 {
    fn new(mut writer: WriteHalf<TcpStream>) -> Self {
        let (tx, mut rx): (mpsc::Sender<Bytes>, _) = mpsc::channel(32);
        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                if let Err(e) = writer.write_all(&data).await {
                    eprintln!("Write error: {}", e);
                    break;
                }
            }

            let _ = writer.shutdown().await;
        });

        AutoShutdownWriteHalfV2 { sender: tx }
    }

    async fn write_all(&mut self, data: &[u8]) -> io::Result<()> {
        // io::copy(&mut data, &mut self.sender).await
        self.sender
            .send(Bytes::copy_from_slice(data))
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, str::from_utf8(&e.0).unwrap()))
    }
}

#[allow(unused)]
async fn client_v1() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, wr) = io::split(socket);
    // let (mut rd, mut wr) = socket.into_split();

    // let mut wr = AutoShutdownWriteHalf::new(wr);
    let mut wr = AutoShutdownWriteHalfV2::new(wr);
    // Write data in the background
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        // Sometimes, the rust type inferencer needs
        // a little help
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        let s = str::from_utf8(&buf[..n]).expect("Invalid UTF-8");
        println!("GOT {:?}", s);
    }

    Ok(())
}

#[allow(unused)]
async fn client_v2() -> io::Result<()> {
    let client = TcpStream::connect("127.0.0.1:6142")
        .await
        .expect("Failed to connect to 127.0.0.1:6142");
    let args = std::env::args().collect::<Vec<String>>().join(" ");
    let (mut rd, mut wr) = client.into_split();
    tokio::spawn(async move {
        wr.write_all(args.as_bytes())
            .await
            .expect("Failed to write content");
    });

    let mut resp = String::new();
    rd.read_to_string(&mut resp)
        .await
        .expect("Failed to read response");
    println!("{}", resp);

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    client_v1().await?;
    client_v2().await?;
    Ok(())
}
