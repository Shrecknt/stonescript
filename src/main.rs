use std::{fs, path::PathBuf};
use clap::Parser;
use crate::{config::ProjectConfig, tokenizer::tokenize};

mod config;
mod stream;
mod tokenizer;
mod tokens;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root of the program to compile
    #[arg(short, long, default_value = "./")]
    root: PathBuf,
    /// Entrypoint file
    #[arg(short, long, default_value = "src/main.ss")]
    entrypoint: PathBuf,
}

fn main() -> Result<(), eyre::Report> {
    let args = Args::parse();

    println!("Compiling with StoneScript version {}", VERSION);
    println!(
        "{{ root = '{}', entrypoint = '{}' }}",
        args.root.display(),
        args.entrypoint.display()
    );

    let project_config: ProjectConfig =
        toml::from_str(&fs::read_to_string(args.root.join("stonescript.toml"))?)?;

    println!("package = {:?}\ndependencies = {:?}", project_config.package, project_config.dependencies);

    let entrypoint_contents = fs::read_to_string(args.root.join(args.entrypoint))?;
    let tokenized = tokenize(&mut entrypoint_contents.into())?;

    println!("Tokenized: {:?}", tokenized);

    println!("Tokens:");
    for token in tokenized {
        println!(
            "[{:?} - {:?}] {}",
            token.token_type, token.specific, token.value
        );
    }

    Ok(())
}
