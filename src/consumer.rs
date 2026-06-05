use tokio::sync::{
    broadcast::{error::RecvError, Receiver},
    watch
};
use tokio_util::sync::CancellationToken;
use crate::{NormalisedTick, risk::RiskState};

pub async fn strategy_worker(
    name: &'static str,
    mut tick_rx: Receiver<NormalisedTick>,
    mut risk_rx: watch::Receiver<RiskState>,
    cancel: CancellationToken
) {
    loop {
        tokio::select! {
            tick = tick_rx.recv() => {
                match tick {
                    Ok(tick) => {
                        println!("[{name}] {} @ {:.2}", tick.symbol, tick.mid_price);
                    }
                    Err(RecvError::Lagged(n)) => {
                        println!("[{name}] LAGGED - missed {n} ticks");
                        return;
                    }
                    Err(RecvError::Closed) => break,
                }
            }

            changed = risk_rx.changed() => {
                if changed.is_err() {
                    return;
                }

                if let RiskState::Halted { .. } = &*risk_rx.borrow() {
                    println!("[{name}] risk halt received");
                    return;
                }
            }

            _ = cancel.cancelled() => {
                println!("[{name}] cancelled");
                return;
            }
        }
    }
}