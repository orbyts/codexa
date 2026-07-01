use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        None => {
            println!("{}", codexa::greeting());
            ExitCode::SUCCESS
        }
        Some("--version" | "-V") => {
            println!("codexa {}", codexa::VERSION);
            ExitCode::SUCCESS
        }
        Some("--help" | "-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(argument) => {
            eprintln!("error: unknown argument `{argument}`\n");
            print_help();
            ExitCode::from(2)
        }
    }
}

fn print_help() {
    println!(
        "Codexa {}\n\nA Git-native content compiler.\n\nUSAGE:\n    codexa [OPTIONS]\n\nOPTIONS:\n    -h, --help       Print help\n    -V, --version    Print version",
        codexa::VERSION
    );
}
