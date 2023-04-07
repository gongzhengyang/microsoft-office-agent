mod parse;
mod walk;

use tokio::time::MissedTickBehavior;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let frequency = std::env::var("FREQUENCY")
        .unwrap_or_else(|_| "3".to_owned())
        .parse::<u64>()
        .unwrap();

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(frequency));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
        tracing::info!("begin read office log");
        let results = walk::walk_for_logs().await;
        println!("{results:?}");
    }
}
