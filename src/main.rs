use std::process::ExitCode;

pub fn main() -> ExitCode {
    ExitCode::SUCCESS
    // match Cmd::parse().run() {
    //     Ok(()) => ExitCode::SUCCESS,
    //     Err(e) => {
    //         _ = writeln!(io::stderr(), "\n❌ エラー:\n{:?}", e);
    //         ExitCode::FAILURE
    //     }
    // }
}
