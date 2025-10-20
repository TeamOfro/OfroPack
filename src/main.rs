use std::process::ExitCode;

use clap::Parser;
use processor::{
    cmd::{Cmd, Run},
    constants::Paths,
};

pub fn main() -> ExitCode {
    if !Paths::assets_path().exists() {
        eprint!(
            "Error: 'assets'ディレクトリが存在しません。OfroPackプロジェクトのルートディレクトリでコマンドを実行してください。"
        );
    }

    match Cmd::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
