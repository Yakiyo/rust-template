//! Rust Template repository
//!
//! Copyright Yakiyo - 2023
//! Licensed under MIT

mod cli;

fn main() {
    init();

    cli::parse();
}

/// initialize some configs and stuffs
fn init() {
    use env_logger::{Builder, Env};
    use std::env;

    // init panic handler
    human_panic::setup_panic!();

    // init logger
    Builder::from_env(Env::new().filter("APP_LOG").write_style("APP_LOG_STYLE"))
        .format_timestamp(None)
        .init();
}
