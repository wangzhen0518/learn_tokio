use std::rc::Rc;

use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    console_subscriber::init();
    tokio::spawn(async {
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }
        yield_now().await;
    });
}
