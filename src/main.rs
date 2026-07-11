use std::{fs, path::PathBuf, process::ExitCode};

use codexa::compiler::{SourceRoot, bundle, compile_roots};

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
    if args.is_empty() {
        print_help();
        return Ok(());
    }
    if matches!(args.as_slice(), [flag] if flag == "--version" || flag == "-V") {
        println!("codexa {}", codexa::VERSION);
        return Ok(());
    }
    if matches!(args.as_slice(), [flag] if flag == "--help" || flag == "-h") {
        print_help();
        return Ok(());
    }

    match args[0].as_str() {
        "validate" => validate_command(&args[1..]),
        "build" => build_command(&args[1..]),
        _ => Err("invalid command".into()),
    }
}

fn validate_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let (roots, _) = parse_roots_and_output(args, false)?;
    let documents = compile_roots(&roots)?;
    println!(
        "Validated {} documents across {} repositories.",
        documents.len(),
        roots.len()
    );
    Ok(())
}

fn build_command(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let (roots, output) = parse_roots_and_output(args, true)?;
    let output = output.ok_or("--output is required")?;
    let documents = compile_roots(&roots)?;
    fs::create_dir_all(&output)?;

    write_json(&output.join("bundle.json"), &bundle(&documents))?;
    codexa::adapter::notion::write_bundle(&documents, &output.join("notion"))?;
    codexa::adapter::web::write_bundle(&documents, &output.join("web"))?;

    println!(
        "Built {} documents from {} repositories.",
        documents.len(),
        roots.len()
    );
    println!("Output: {}", output.display());
    Ok(())
}

fn parse_roots_and_output(
    args: &[String],
    require_output: bool,
) -> Result<(Vec<SourceRoot>, Option<PathBuf>), Box<dyn std::error::Error>> {
    let mut roots = Vec::new();
    let mut output = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--source-root" => {
                let path = args.get(index + 1).ok_or("--source-root requires PATH")?;
                let repository = args
                    .get(index + 2)
                    .ok_or("--source-root requires REPOSITORY after PATH")?;
                roots.push(SourceRoot {
                    path: PathBuf::from(path),
                    repository: repository.clone(),
                });
                index += 3;
            }
            "--output" => {
                let path = args.get(index + 1).ok_or("--output requires DIR")?;
                output = Some(PathBuf::from(path));
                index += 2;
            }
            other => return Err(format!("unexpected argument `{other}`").into()),
        }
    }

    if roots.is_empty() {
        return Err("at least one --source-root PATH REPOSITORY is required".into());
    }
    if require_output && output.is_none() {
        return Err("--output is required".into());
    }
    Ok((roots, output))
}

fn write_json<T: serde::Serialize>(
    path: &std::path::Path,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

fn print_help() {
    println!(
        "Codexa {}\n\nGit-native multi-endpoint content compiler.\n\nUSAGE:\n    codexa validate --source-root <PATH> <OWNER/REPO> [--source-root ...]\n    codexa build --source-root <PATH> <OWNER/REPO> [--source-root ...] --output <DIR>\n\nSOURCE DOCUMENTS:\n    Markdown with schema: codexa.document@2\n",
        codexa::VERSION
    );
}
