use std::{fs, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    match args.as_slice() {
        [] => {
            println!("{}", codexa::greeting());
            Ok(())
        }
        [flag] if flag == "--version" || flag == "-V" => {
            println!("codexa {}", codexa::VERSION);
            Ok(())
        }
        [flag] if flag == "--help" || flag == "-h" => {
            print_help();
            Ok(())
        }
        [command, input, adapter_flag, adapter, output_flag, output]
            if command == "build"
                && adapter_flag == "--adapter"
                && adapter == "web"
                && output_flag == "--output" =>
        {
            build_web(PathBuf::from(input), PathBuf::from(output))
        }
        [command, input, output_flag, output]
            if command == "build" && output_flag == "--output" =>
        {
            build_web(PathBuf::from(input), PathBuf::from(output))
        }
        [
            command,
            input,
            output_flag,
            output,
            repository_flag,
            repository,
            source_path_flag,
            source_path,
        ] if command == "build-notion"
            && output_flag == "--output"
            && repository_flag == "--repository"
            && source_path_flag == "--source-path" =>
        {
            build_notion(
                PathBuf::from(input),
                PathBuf::from(output),
                repository,
                source_path,
            )
        }
        _ => {
            print_help();
            Err("invalid arguments".into())
        }
    }
}

fn build_web(input: PathBuf, output: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let markdown = fs::read_to_string(&input)?;
    let document = codexa::parser::parse_markdown(&markdown)?;
    let source_name = input
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("document.md");

    codexa::adapter::web::write_artifact(document, source_name, &output)?;
    println!("Web artifact written to {}", output.display());
    Ok(())
}

fn build_notion(
    input: PathBuf,
    output: PathBuf,
    repository: &str,
    source_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let markdown = fs::read_to_string(&input)?;

    codexa::adapter::notion::write_artifact(&markdown, repository, source_path, &output)?;

    println!("Notion artifact written to {}", output.display());
    Ok(())
}

fn print_help() {
    println!(
        "Codexa {}\n\nA Git-native content compiler.\n\nUSAGE:\n    codexa [OPTIONS]\n    codexa build <INPUT> [--adapter web] --output <DIR>\n    codexa build-notion <INPUT> --output <DIR> --repository <OWNER/REPO> --source-path <PATH>\n\nOPTIONS:\n    -h, --help       Print help\n    -V, --version    Print version",
        codexa::VERSION
    );
}
