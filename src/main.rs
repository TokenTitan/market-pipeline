use phase1_market_pipeline::*;
use tokio::sync::{mpsc, broadcast};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel(100);

    let tx_binance = tx.clone();
    let tx_okx = tx.clone();
    let tx_kraken = tx.clone();
    drop(tx);

    let (bcast_tx, _) = broadcast::channel::<NormalisedTick>(256);

    let rx_a = bcast_tx.subscribe();
    let rx_b = bcast_tx.subscribe();

    let (risk_tx, risk_rx) = tokio::sync::watch::channel(risk::RiskState::Normal);
    let cancel = CancellationToken::new();

    let binance_producer = tokio::spawn(
        producer::feed_producer(
            "Binance",
            tx_binance,
            50
        )
    );

    let okx_producer = tokio::spawn(
        producer::feed_producer(
            "OKX",
            tx_okx,
            50
        )
    );

    let kraken_producer = tokio::spawn(
        producer::feed_producer(
            "Kraken",
            tx_kraken,
            50
        )
    );

    let normaliser = tokio::spawn(
        normalizer::normalizer(
            rx,
            bcast_tx
        )
    );

    let consumer_a = tokio::spawn(
        consumer::strategy_worker("Strategy A", rx_a, risk_rx.clone(), cancel.clone())
    );

    let consumer_b = tokio::spawn(
        consumer::strategy_worker("Strategy B", rx_b, risk_rx.clone(), cancel.clone())
    );

    let risk_monitor = tokio::spawn(
        risk::risk_monitor(risk_tx, cancel.clone())
    );

    drop(cancel);

    let _ = tokio::join!(
        binance_producer,
        okx_producer,
        kraken_producer,
        normaliser,
        consumer_a,
        consumer_b,
        risk_monitor
    );

    Ok(())
}
