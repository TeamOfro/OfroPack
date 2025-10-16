#![allow(clippy::single_component_path_imports)]

use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;

mod cmd;
mod constants;
mod file_utils;
mod gallery;
mod image_validator;
mod models;
mod processor;
mod runner;

use crate::cmd::{Cmd, Run};

pub fn main() -> ExitCode {
    match Cmd::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            _ = writeln!(io::stderr(), "\n❌ エラー:\n{:?}", e);
            ExitCode::FAILURE
        }
    }
}
