#[macro_use]
extern crate slog;
extern crate slog_json_concur;

use slog::Drain;

// An original version uses slog-async to synchronize drain.
// This version does not require that.

fn main() {
    let drain = slog_json_concur::Json::new(std::io::stdout())
        .set_pretty(true)
        .add_default_keys()
        .build()
        .fuse();
    let log = slog::Logger::root(drain, o!("format" => "pretty"));

    info!(log, "An example log message"; "foo" => "bar");
    info!(log, "Another example log message"; "fizz" => "buzz");
}
