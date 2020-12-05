mod fib_retry;
mod s3;
use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};

fn main() {
    let counter_opts = Opts::new("TestCounter", "test counter helper");
    let counter = Counter::with_opts(counter_opts).unwrap();
    let r = Registry::new();
    r.register(Box::new(counter.clone())).unwrap();
    counter.inc();

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_family = r.gather();
    encoder.encode(&metric_family, &mut buffer).unwrap();

    println!("{}", String::from_utf8(buffer).unwrap())
}
