#![allow(dead_code)]

use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use tokio::sync::{mpsc, oneshot};

async fn select_basic_usage() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 =>{
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

async fn some_operation() -> &'static str {
    tokio::time::sleep(Duration::from_millis(10)).await;
    "some operation"
}

async fn cancel_future() {
    let (tx1, mut rx1): (mpsc::Sender<&str>, _) = mpsc::channel(1);
    let (tx2, mut rx2): (mpsc::Sender<&str>, _) = mpsc::channel(1);

    tokio::spawn(async move {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val).await;
            }
            _ = async {} => {drop(tx1);}
        }
    });

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(20)).await;
        let _ = tx2.send("two").await;
    });

    tokio::select! {
        val = rx1.recv() =>{
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2.recv() => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

async fn my_select_fn() {
    struct MySelect {
        rx1: mpsc::Receiver<&'static str>,
        rx2: mpsc::Receiver<&'static str>,
    }

    impl Future for MySelect {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // {
            //     let recv1 = self.rx1.recv();
            //     tokio::pin!(recv1);
            //     if let Poll::Ready(val) = recv1.poll(cx) {
            //         println!("rx1 completed first with {:?}", val);
            //         return Poll::Ready(());
            //     }
            // }
            // {
            //     let recv2 = self.rx2.recv();
            //     tokio::pin!(recv2);
            //     if let Poll::Ready(val) = recv2.poll(cx) {
            //         println!("rx2 completed first with {:?}", val);
            //         return Poll::Ready(());
            //     }
            // }

            if let Poll::Ready(val) = self.rx1.poll_recv(cx) {
                println!("rx1 completed first with {:?}", val);
                return Poll::Ready(());
            } else if let Poll::Ready(val) = self.rx2.poll_recv(cx) {
                println!("rx2 completed first with {:?}", val);
                return Poll::Ready(());
            }

            Poll::Pending
        }
    }

    let (tx1, rx1): (mpsc::Sender<&str>, _) = mpsc::channel(1);
    let (tx2, rx2): (mpsc::Sender<&str>, _) = mpsc::channel(1);

    tokio::spawn(async move {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val).await;
            }
            _ = async {} => {drop(tx1);}
        }
    });

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(20)).await;
        let _ = tx2.send("two").await;
    });

    MySelect { rx1, rx2 }.await;
}

async fn pattern_match() {
    let (tx1, mut rx1): (mpsc::Sender<&str>, _) = mpsc::channel(128);
    let (tx2, mut rx2): (mpsc::Sender<&str>, _) = mpsc::channel(128);

    tokio::spawn(async move {
        // let _ = tx1.send("one").await;
        // let _ = tx2.send("two").await;
        tx1.closed().await;
        tx2.closed().await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;
    tokio::select! {
        val = rx1.recv() => {
            println!("Got {:?} from rx1", val);
        }
        val = rx2.recv() => {
            println!("Got {:?} from rx2", val);
        }
        else => {
            println!("Both channels closed");
        }
    }
}

async fn loop_select() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    tokio::spawn(async move {
        let _ = tx1.send("1").await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        let _ = tx2.send("2").await;
        let _ = tx3.send("3").await;
    });

    tokio::time::sleep(Duration::from_millis(50)).await;
    #[allow(clippy::never_loop)]
    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => break,
        };
        println!("Got {:?}", msg);
    }

    println!("All channels have been closed.");
}

async fn action(input: Option<i32>) -> Option<i32> {
    input
}

async fn resume_async() {
    let (tx, mut rx): (mpsc::Sender<i32>, _) = mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(2).await;
        let _ = tx.send(3).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                if let Some(val) = res {
                    println!("GOT = {}",val);
                    return ;
                }
            },
            Some(val) = rx.recv() => {
                if val % 2 == 0 {
                    operation.set(action(Some(val)));
                    done = false;
                }
            }
            else => break,
        }
    }
}

#[tokio::main]
async fn main() {
    // select_basic_usage().await;
    // cancel_future().await;
    // my_select_fn().await;
    // pattern_match().await;
    // loop_select().await;
    resume_async().await;
}
