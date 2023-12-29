use prometheus::{Encoder, IntCounter, Gauge, Opts};
use lazy_static::lazy_static;

lazy_static! {
    static ref METRICS_COUNTER: IntCounter = IntCounter::new("http_metrics_counter", "How many times have metrics been scraped").unwrap();
    static ref MSG_COUNT: IntCounter = IntCounter::new("http_message_counter", "How many messages have been sent").unwrap();
    static ref CLIENT_COUNT: Gauge = Gauge::with_opts(Opts::new("http_client_gauge", "How many clients are currently connected")).expect("metric can be created");
}

pub fn get_metrics() -> Vec<u8> {
    METRICS_COUNTER.inc();
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();

    let metric_families = prometheus::default_registry().gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    buffer
}


pub fn inc_msg_count() {
    MSG_COUNT.inc();
}

pub fn inc_client_count() {
    CLIENT_COUNT.inc();
}

pub fn dec_client_count() {
    CLIENT_COUNT.dec();
}


pub async fn init_counters() {
    prometheus::default_registry()
    .register(Box::new(METRICS_COUNTER.clone()))
    .expect("Failed to register metrics counter");
    prometheus::default_registry()
    .register(Box::new(MSG_COUNT.clone()))
    .expect("Failed to register message counter");
    prometheus::default_registry()
    .register(Box::new(CLIENT_COUNT.clone()))
    .expect("Failed to register client counter");
}
