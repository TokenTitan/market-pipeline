use tokio::sync::mpsc::{Receiver};
use tokio::sync::broadcast::{Sender};
use crate::state::{RawTick, NormalisedTick};
use crate::helper::now_ns;

pub async fn normalizer(mut rx: Receiver<RawTick>, tx_out: Sender<NormalisedTick>) {
    while let Some(tick) = rx.recv().await {
        let normalised_tick = NormalisedTick {
            symbol: tick.symbol,
            mid_price: (tick.bid + tick.ask) / 2.0,
            spread: tick.ask - tick.bid,
            source_exchange: tick.exchange,
            pipeline_latency_us: now_ns().saturating_sub(tick.received_ns) / 1000,
        };

        println!(
            "[{}] [normalizer] {} mid={:.2} spread={:.4} latency={}µs",
            tick.exchange,
            normalised_tick.symbol,
            normalised_tick.mid_price,
            normalised_tick.spread,
            normalised_tick.pipeline_latency_us
        );

        if tx_out.send(normalised_tick).is_err() {
            println!("[normalizer] output channel closed");
            return;
        }
    }

    println!("[normalizer] done");
}
