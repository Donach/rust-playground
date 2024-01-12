use lazy_static::lazy_static;
use prometheus::{Encoder, Gauge, IntCounter, Opts, TextEncoder};

lazy_static! {
    static ref METRICS_COUNTER: IntCounter = IntCounter::new(
        "http_metrics_counter",
        "How many times have metrics been scraped"
    )
    .unwrap();
    static ref MSG_COUNT: IntCounter =
        IntCounter::new("http_message_counter", "How many messages have been sent").unwrap();
    static ref CLIENT_COUNT: Gauge = Gauge::with_opts(Opts::new(
        "http_client_gauge",
        "How many clients are currently connected"
    ))
    .expect("metric can be created");
}

pub async fn get_metrics() -> Result<String, prometheus::Error> {
    METRICS_COUNTER.inc();
    let encoder = TextEncoder::new();
    let mut buffer = vec![];

    let metric_families = prometheus::gather();
    encoder
        .encode(&metric_families, &mut buffer)
        .map(move |()| String::from_utf8(buffer).expect("invalid utf8 in metrics"))
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

pub fn init_counters() {
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
