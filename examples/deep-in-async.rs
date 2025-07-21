#![allow(dead_code)]

use std::{
    pin::Pin,
    sync::{Arc, Mutex, mpsc},
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

use futures::task::{self, ArcWake};
use tokio::sync::Notify;

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
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = Instant::now();
        if Instant::now() >= self.when {
            Poll::Ready("Hello world")
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

async fn delay(when: Instant) {
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();
        if now < when {
            thread::sleep(when - now);
        }
        notify_clone.notify_one();
    });
    notify.notified().await;
}

struct TaskFuture {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    poll: Poll<()>,
}

impl TaskFuture {
    fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
        TaskFuture {
            future: Box::pin(future),
            poll: Poll::Pending,
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if self.poll.is_pending() {
            self.poll = self.future.as_mut().poll(cx);
        }
        self.poll
    }
}

struct Task {
    task_future: Mutex<TaskFuture>,
    executor: mpsc::Sender<Arc<Task>>,
}

impl Task {
    fn new(
        future: impl Future<Output = ()> + Send + 'static,
        executor: mpsc::Sender<Arc<Task>>,
    ) -> Self {
        Task {
            task_future: Mutex::new(TaskFuture::new(future)),
            executor,
        }
    }

    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }

    fn spawn(future: impl Future<Output = ()> + Send + 'static, sender: mpsc::Sender<Arc<Task>>) {
        let task = Arc::new(Task::new(future, sender));
        task.schedule();
    }

    fn poll(self: &Arc<Self>) -> Poll<()> {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut task_future = self.task_future.try_lock().unwrap();
        task_future.poll(&mut cx)
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

struct Executor {
    scheduled: mpsc::Receiver<Arc<Task>>,
    // sender: mpsc::Sender<Arc<Task>>,
    sender: Option<mpsc::Sender<Arc<Task>>>,
    task_cnt: u64,
}

impl Executor {
    fn new() -> Self {
        let (sender, scheduled) = mpsc::channel();
        let sender = Some(sender);
        Executor {
            sender,
            scheduled,
            task_cnt: 0,
        }
    }

    fn spawn(&mut self, future: impl Future<Output = ()> + Send + 'static) {
        let sender = self.sender.as_ref().unwrap();
        Task::spawn(future, sender.clone());
        self.task_cnt += 1;
    }

    fn run(&mut self) {
        while let Ok(task) = self.scheduled.recv() {
            if task.poll().is_ready() {
                self.task_cnt -= 1;
                if self.task_cnt == 0 {
                    self.sender = None;
                }
            }
        }
    }
}

fn main() {
    let mut mini_tokio = Executor::new();
    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(1);
        let future = Delay::new(when);
        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_secs(1);
        let future = Delay::new(when);
        let out = future.await;
        assert_eq!(out, "done");
    });

    mini_tokio.run();
}
