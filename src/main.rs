#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_async;

use slog::{Error};
use std::fs::File;
use std::io;
use std::io::BufRead;
use crate::slog::Drain;

fn log() -> Result<slog::Logger, Error> {
    let log_file = File::create("app.log")?;
    let drain = slog_json::Json::new(log_file)
        .set_pretty(false)
        .add_default_keys()
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

    Ok(log)
}

fn main() -> Result<(), io::Error> {
    let writer = log()?;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        info!(writer, "{}", l);
    }

    Ok(())
}
