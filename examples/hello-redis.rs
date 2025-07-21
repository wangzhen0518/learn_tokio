use std::time::Duration;

use tokio::time::sleep;

async fn mini_redis_demo() -> mini_redis::Result<()> {
    let mut client = mini_redis::client::connect("127.0.0.1:6379")
        .await
        .expect("Bind port 6379 failed");
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    if let Some(content) = result {
        println!(
            "got value from the server: result={:?}",
            str::from_utf8(&content).expect("Invalid UTF-8")
        );
    }
    Ok(())
}

async fn say_world() {
    let op = mini_redis_demo();
    println!("world");
    op.await.expect("Failed to run mini redis demo");
}

#[tokio::main]
async fn main() {
    let op = say_world();

    sleep(Duration::from_millis(100)).await;
    println!("hello");

    op.await;

    // tokio::join!(say_world(), sleep(Duration::from_secs(1)));

    async { println!("hello 2") }.await;

    say_world().await;

    let handler = tokio::spawn(say_world());

    async {
        sleep(Duration::from_millis(100)).await;
        println!("hello");
    }
    .await;

    handler.await.expect("Failed to run say_world");
}

// fn main() {
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         let op = say_world();
//         println!("hello");
//         op.await;
//     });
// }
