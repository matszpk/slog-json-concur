# slog-json-concur

[![Crate version](https://img.shields.io/crates/v/slog-json-concur)](https://crates.io/crates/slog-json-concur)
[![Docs](https://docs.rs/slog-json-concur/badge.svg)](https://docs.rs/slog-json-concur)

A simple fork of the slog-json 2.6.1 (original version by Dawid Ciężarkiewicz)
crate which introduces a buffering and concurrency while generating output of a log record.

An original drain requires mutex before using. This fork introduces a buffering
before final write and it uses mutexes only on same final write, making same logging
as concurrent and parallel.

Original slog-json: https://crates.io/crates/slog-json, https://github.com/slog-rs/json.
