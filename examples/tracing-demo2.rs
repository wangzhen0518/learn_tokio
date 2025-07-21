use tracing::{Level, debug, error, info, info_span, instrument, warn};

#[instrument] // 自动为函数创建 span
fn process_data(data: &str) {
    // debug!("Processing data: {}", data);

    if data.is_empty() {
        // warn!("Received empty data");
        return;
    }

    // 创建子 span
    // let _span = info_span!("validation").entered();
    validate(data);

    info!("Data processed successfully");
}

fn validate(data: &str) {
    if data.len() < 5 {
        error!("Data too short: {} characters", data.len());
    } else {
        debug!("Validation passed");
    }
}

#[tracing::instrument] // 设置函数级 span
fn main() {
    // 设置 tracing 订阅器（支持多输出）
    // tracing_subscriber::registry()
    //     // 终端日志输出（带颜色）
    //     .with(fmt::layer().with_target(false).pretty())
    //     // 环境变量过滤日志级别
    //     .init();

    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_thread_ids(true)
        .pretty()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    // 事件记录 (无上下文)
    tracing::event!(Level::INFO, "Starting application");

    // 手动创建顶级 span
    let root_span = info_span!("main_operation", app_version = "1.0");
    let _guard = root_span.enter();

    // 跨函数跟踪
    process_data("Hello");
    process_data("");
    process_data("Hi");

    info!("Main operation completed");
}
