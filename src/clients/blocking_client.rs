#![allow(dead_code)]

use std::{
    error::Error,
    fmt::Display,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};

use bytes::Bytes;
use mini_redis::client::{Client, Message, Subscriber};
use tokio::{net::ToSocketAddrs, runtime::Runtime, sync::mpsc};

#[derive(Debug)]
struct MyError {}
impl MyError {
    fn new() -> Self {
        MyError {}
    }
}
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "MyError")
    }
}
impl Error for MyError {}

pub struct BlockingClient {
    inner: Client,
    rt: Runtime,
}

impl BlockingClient {
    pub fn connect<T: ToSocketAddrs>(addr: T) -> mini_redis::Result<BlockingClient> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let inner = rt.block_on(mini_redis::client::connect(addr))?;
        Ok(BlockingClient { inner, rt })
    }

    pub fn get(&mut self, key: &str) -> mini_redis::Result<Option<Bytes>> {
        self.rt.block_on(self.inner.get(key))
    }

    pub fn set(&mut self, key: &str, value: Bytes) -> mini_redis::Result<()> {
        self.rt.block_on(self.inner.set(key, value))
    }

    pub fn set_expires(
        &mut self,
        key: &str,
        value: Bytes,
        expiration: Duration,
    ) -> mini_redis::Result<()> {
        self.rt
            .block_on(self.inner.set_expires(key, value, expiration))
    }

    pub fn publish(&mut self, channel: &str, message: Bytes) -> mini_redis::Result<u64> {
        self.rt.block_on(self.inner.publish(channel, message))
    }

    pub fn subscribe(self, channels: Vec<String>) -> mini_redis::Result<BlockingSubscriber> {
        let subscriber = self.rt.block_on(self.inner.subscribe(channels))?;
        Ok(BlockingSubscriber {
            inner: subscriber,
            rt: self.rt,
        })
    }
}

pub struct BlockingSubscriber {
    inner: Subscriber,
    rt: Runtime,
}

impl BlockingSubscriber {
    pub fn get_subscribed(&self) -> &[String] {
        self.inner.get_subscribed()
    }

    pub fn next_message(&mut self) -> mini_redis::Result<Option<Message>> {
        self.rt.block_on(self.inner.next_message())
    }

    pub fn subscribe(&mut self, channels: &[String]) -> mini_redis::Result<()> {
        self.rt.block_on(self.inner.subscribe(channels))
    }

    pub fn unsubscribe(&mut self, channels: &[String]) -> mini_redis::Result<()> {
        self.rt.block_on(self.inner.unsubscribe(channels))
    }
}

impl IntoIterator for BlockingSubscriber {
    type Item = mini_redis::Result<Message>;
    type IntoIter = SubscriberIterator;
    fn into_iter(self) -> Self::IntoIter {
        SubscriberIterator {
            inner: self.inner,
            rt: self.rt,
        }
    }
}

pub struct SubscriberIterator {
    inner: Subscriber,
    rt: Runtime,
}

impl Iterator for SubscriberIterator {
    type Item = mini_redis::Result<Message>;
    fn next(&mut self) -> Option<Self::Item> {
        self.rt.block_on(self.inner.next_message()).transpose()
    }
}

fn spawn_new_thread() {
    async fn my_bg_task(i: u64) {
        let millis = 1000 - 50 * i;
        println!("Task {} sleeping for {} ms", i, millis);
        tokio::time::sleep(Duration::from_millis(millis)).await;
        println!("Task {} finished", i);
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(rt.spawn(my_bg_task(i)));
    }

    thread::sleep(Duration::from_millis(750));
    println!("Finished time-consuming task.");

    for handle in handles {
        rt.block_on(handle).unwrap();
    }
}

fn message_passing() {
    pub struct Task {
        name: String,
    }

    async fn handle_task(task: Task) {
        println!("Got task {}", task.name);
    }

    #[derive(Clone)]
    pub struct TaskSpawner {
        spawn: Option<mpsc::Sender<Task>>,
        handler: Option<Arc<JoinHandle<()>>>,
    }

    impl TaskSpawner {
        pub fn new() -> Self {
            let (send, mut recv) = mpsc::channel(128);
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap();
            let handler = thread::spawn(move || {
                rt.block_on(async move {
                    while let Some(task) = recv.recv().await {
                        // tokio::time::sleep(Duration::from_millis(100)).await;
                        tokio::spawn(handle_task(task));
                    }
                });
            });

            TaskSpawner {
                spawn: Some(send),
                handler: Some(Arc::new(handler)),
            }
        }

        pub fn spawn(&self, task: Task) {
            self.spawn
                .as_ref()
                .unwrap()
                .blocking_send(task)
                .expect("The shared runtime has shutdown");
        }
    }

    impl Drop for TaskSpawner {
        fn drop(&mut self) {
            drop(self.spawn.take().unwrap());
            if Arc::strong_count(self.handler.as_ref().unwrap()) == 1 {
                if let Ok(handler) = Arc::try_unwrap(self.handler.take().unwrap()) {
                    let _ = handler.join();
                }
            }
        }
    }

    let ts = TaskSpawner::new();
    ts.spawn(Task {
        name: "Wang Zhen 1".to_string(),
    });
    ts.spawn(Task {
        name: "Wang Zhen 2".to_string(),
    });
    // thread::sleep(Duration::from_millis(100));
}

fn main() {
    // spawn_new_thread();
    message_passing();
}
