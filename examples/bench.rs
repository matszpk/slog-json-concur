use slog::{info, Drain, Logger, Never, SendSyncRefUnwindSafeDrain};
use std::io::sink;
use std::panic::UnwindSafe;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn run_bench<D>(name: &str, drain: D, iters: u64, cpus: usize)
where
    D: Drain + 'static + UnwindSafe + SendSyncRefUnwindSafeDrain<Err = Never, Ok = ()>,
{
    println!("Starting {}", name);
    let logger = Logger::root(drain, slog::o!());
    let mut join_handles = vec![];
    let logger = Arc::new(logger);
    let now = Instant::now();
    for i in 0..cpus {
        let logger = logger.clone();
        join_handles.push(thread::spawn(move || {
            for j in 0..iters {
                info!(logger, "Simple text to log";
                        "iter" => j, "cpu" => i, "integer" => 16, "float" => 123.2f32);
            }
        }));
    }
    join_handles.drain(..).for_each(|h| h.join().unwrap());
    let dur = now.elapsed().as_secs_f64();
    println!(
        "Result for {}: time = {}s, ns/iter = {}",
        name,
        dur,
        1e9 * dur / ((iters * (cpus as u64)) as f64)
    );
}

fn main() {
    let mut args = std::env::args().skip(1);
    let iters = args
        .next()
        .map(|x| x.parse().expect("Expected iterations number"))
        .unwrap_or(1000000);
    let cpus = args
        .next()
        .map(|x| x.parse().expect("Expected CPU's number"))
        .unwrap_or(num_cpus::get());

    run_bench(
        "slog-json-concur",
        slog_json_concur::Json::new(sink())
            .add_default_keys()
            .build()
            .fuse(),
        iters,
        cpus,
    );
    run_bench(
        "slog-json",
        Mutex::new(slog_json::Json::new(sink()).add_default_keys().build()).fuse(),
        iters,
        cpus,
    );
}
