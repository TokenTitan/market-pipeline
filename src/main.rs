use phase1_market_pipeline::*;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(100);
    let (tx_norm, _rx_norm) = mpsc::channel(100);

    let h_producer = tokio::spawn(
        producer::feed_producer(
            "Uniswap",
            tx,
            10
        )
    );

    let h_normaliser = tokio::spawn(
        normalizer::normalizer(
            rx,
            tx_norm
        )
    );

    let _ = tokio::join!(h_producer, h_normaliser);

    Ok(())
}
