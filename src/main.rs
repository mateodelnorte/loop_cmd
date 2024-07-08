use loop_lib::args::parse_args;
use loop_lib::run;

fn main() -> std::process::ExitCode {
    let args = parse_args();
    let exit_code = run(args);
    std::process::ExitCode::from(exit_code as u8)
}