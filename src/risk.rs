use std::time::Duration;

use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq)]
pub enum RiskState {
    Normal,
    Halted { reason: String },
}

pub async fn risk_monitor(tx: watch::Sender<RiskState>, cancel: CancellationToken) {
    tokio::time::sleep(Duration::from_millis(25)).await;

    let _ = tx.send(RiskState::Halted {
        reason: "position limit breached".to_string(),
    });

    cancel.cancel();
}
