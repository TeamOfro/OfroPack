use std::process::ExitCode;

use clap::Parser;
use processor::{
    cmd::{Cmd, Run},
    paths::Paths,
};

pub fn main() -> ExitCode {
    if !Paths::assets_path().exists() {
        eprint!(
            "エラー: 'assets' ディレクトリが存在しません。OfroPackプロジェクトのルートディレクトリでコマンドを実行してください。"
        );
    }

    match Cmd::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("エラー: {e}");
            ExitCode::FAILURE
        }
    }
}
