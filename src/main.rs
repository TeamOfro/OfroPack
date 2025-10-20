use std::process::ExitCode;

use clap::Parser;
use processor::cmd::{Cmd, Run};

pub fn main() -> ExitCode {
    match Cmd::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
