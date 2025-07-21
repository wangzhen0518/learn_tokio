#![allow(dead_code)]

use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

// use futures::Stream;
use tokio_stream::{Stream, StreamExt};

async fn tokio_stream_demo() {
    let mut stream = tokio_stream::iter(&[1, 2, 3, 4]);
    while let Some(x) = stream.next().await {
        println!("{}", x);
    }
}

async fn stream_publish() {
    async fn publish() -> mini_redis::Result<()> {
        let mut client = mini_redis::client::connect("127.0.0.1:6379").await?;
        client.publish("number", "1".into()).await?;
        client.publish("number", "two".into()).await?;
        client.publish("number", "3".into()).await?;
        client.publish("number", "four".into()).await?;
        client.publish("number", "5".into()).await?;
        client.publish("number", "six".into()).await?;
        client.publish("number", "7".into()).await?;

        Ok(())
    }

    async fn subscribe() -> mini_redis::Result<()> {
        let client = mini_redis::client::connect("127.0.0.1:6379").await?;
        let subscribe = client.subscribe(vec!["number".to_string()]).await?;
        let messages = subscribe
            .into_stream()
            .filter_map(|msg| match msg {
                Ok(msg) if msg.content.len() == 1 => Some(
                    msg.content,
                    // String::from_utf8(msg.content.to_vec())
                    //     .unwrap()
                    //     .parse::<u32>()
                    //     .unwrap(),
                ),
                _ => None,
            })
            .take(2);
        tokio::pin!(messages);
        while let Some(msg) = messages.next().await {
            println!("Message: {:?}", msg);
        }
        Ok(())
    }

    tokio::spawn(publish());

    let _ = subscribe().await;
}

async fn interval_stream() {
    struct Delay {
        when: Instant,
        waker: Option<Arc<Mutex<Waker>>>,
    }

    impl Delay {
        fn new(when: Instant) -> Self {
            Delay { when, waker: None }
        }
    }

    impl Future for Delay {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let now = Instant::now();
            if Instant::now() >= self.when {
                Poll::Ready(())
            } else if let Some(waker) = &self.waker {
                let mut waker = waker.lock().unwrap();
                if !waker.will_wake(cx.waker()) {
                    *waker = cx.waker().clone();
                }
                Poll::Pending
            } else {
                let waker = Arc::new(Mutex::new(cx.waker().clone()));
                let delta = self.when - now;
                self.waker = Some(waker.clone());
                thread::spawn(move || {
                    thread::sleep(delta);
                    waker.lock().unwrap().wake_by_ref();
                });
                Poll::Pending
            }
        }
    }

    struct Interval {
        interval: Duration,
        repeat: u64,
        cnt: u64,
        delay: Delay,
    }

    impl Interval {
        fn new(interval: Duration, repeat: u64) -> Self {
            Interval {
                interval,
                repeat,
                cnt: 0,
                delay: Delay::new(Instant::now()),
            }
        }
    }
    impl Stream for Interval {
        type Item = u64;
        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if self.cnt < self.repeat {
                if Pin::new(&mut self.delay).poll(cx).is_ready() {
                    let delay = Delay::new(self.delay.when + self.interval);
                    self.delay = delay;
                    self.cnt += 1;
                    Poll::Ready(Some(self.cnt))
                } else {
                    Poll::Pending
                }
            } else {
                Poll::Ready(None)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let rem = self.repeat - self.cnt;
            (rem as usize, Some(rem as usize))
        }
    }

    let mut itv = Interval::new(Duration::from_millis(100), 3);
    while let Some(x) = itv.next().await {
        println!("{}", x);
    }
}

#[tokio::main]
async fn main() {
    stream_publish().await;
    interval_stream().await;
}
