#[derive(Clone, Debug)]
pub struct RawTick {
    pub exchange: &'static str,
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub received_ns: u64,
}

#[derive(Clone, Debug)]
pub struct NormalisedTick {
    pub symbol: String,
    pub mid_price: f64,
    pub spread: f64,
    pub source_exchange: &'static str,
    pub pipeline_latency_us: u64
}
