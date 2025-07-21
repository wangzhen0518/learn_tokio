#[tracing::instrument]
fn trace_me(a: u32, b: u32) -> u32 {
    tracing::debug!("hello, {}, {}", a, b);
    a + b
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // let subscriber = tracing_subscriber::fmt()
    //     .compact()
    //     .with_file(true)
    //     .with_line_number(true)
    //     .with_target(true)
    //     .with_thread_ids(true)
    //     .with_thread_names(true)
    //     .pretty()
    //     .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    trace_me(1, 2);
}
