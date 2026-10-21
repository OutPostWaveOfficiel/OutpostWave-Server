//! `outpostwave-server` entry point.

use std::path::PathBuf;
use std::process::ExitCode;

use ow_server::config::Config;

const DEFAULT_CONFIG: &str = "config/config.toml";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut path = PathBuf::from(DEFAULT_CONFIG);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--config" | "-c" => match args.next() {
                Some(p) => path = PathBuf::from(p),
                None => {
                    eprintln!("error: --config needs a path");
                    return ExitCode::FAILURE;
                }
            },
            "--version" | "-V" => {
                println!("outpostwave-server {}", env!("CARGO_PKG_VERSION"));
                return ExitCode::SUCCESS;
            }
            "--help" | "-h" => {
                println!("usage: outpostwave-server [--config <path>] [--version]");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("error: unknown argument {other:?}");
                return ExitCode::FAILURE;
            }
        }
    }

    let cfg = match Config::load(&path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("error: {}: {e}", path.display());
            return ExitCode::FAILURE;
        }
    };
    println!(
        "outpostwave-server {} — \"{}\" ({:?}) on {}",
        env!("CARGO_PKG_VERSION"),
        cfg.server.name,
        cfg.server.mode,
        cfg.server.bind
    );
    // The network stack (API, gateway) is added in roadmap step 6.
    ExitCode::SUCCESS
}
