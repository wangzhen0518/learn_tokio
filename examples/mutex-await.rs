use std::{sync::Mutex, time::Duration};

struct CanIncrement {
    mutex: Mutex<i32>,
}

impl CanIncrement {
    fn new() -> CanIncrement {
        CanIncrement {
            mutex: Mutex::new(0),
        }
    }
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

async fn do_something_async() {
    tokio::time::sleep(Duration::from_millis(100)).await
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    do_something_async().await;
}

#[tokio::main]
async fn main() {
    let can_incre = CanIncrement::new();
    increment_and_do_stuff(&can_incre).await;
    tokio::spawn(async move {
        increment_and_do_stuff(&can_incre).await;
    });
}
