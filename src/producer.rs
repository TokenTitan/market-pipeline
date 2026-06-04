use tokio::sync::mpsc::Sender;
use tokio::time::{sleep, Duration};

use crate::{now_ns, RawTick};

pub async fn feed_producer(exchange: &'static str, tx: Sender<RawTick>, count: u32) {
    for i in 0..count {
        let bid = 69_000.0 + (i as f64 * 0.5);
        let tick = RawTick {
            exchange,
            symbol: "BTC-USD".to_string(),
            bid,
            ask: bid + 1.0,
            received_ns: now_ns(),
        };

        if tx.send(tick).await.is_err() {
            println!("[{exchange}] channel closed");
            return;
        }

        sleep(Duration::from_millis(1)).await;
    }

    println!("[{exchange}] done");
}
