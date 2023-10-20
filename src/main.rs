use std::fs;

use clap::Parser;
use toml::Table;

use crate::tokenizer::tokenize;

mod stream;
mod tokenizer;
mod tokens;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root of the program to compile
    #[arg(short, long, default_value_t = String::from("./"))]
    root: String,
    /// Entrypoint file
    #[arg(short, long, default_value_t = String::from("src/main.ss"))]
    entrypoint: String,
}

fn main() -> Result<(), eyre::Report> {
    let args = Args::parse();

    println!("Compiling with StoneScript version {}", VERSION);
    println!(
        "{{ root = '{}', entrypoint = '{}' }}",
        args.root, args.entrypoint
    );

    let project_config = fs::read_to_string(format!("{}/stonescript.toml", args.root))?;
    let project_config = project_config.parse::<Table>()?;
    let package = project_config["package"]
        .as_table()
        .unwrap()
        .iter()
        .map(|f| (f.0.as_str(), f.1.as_str().unwrap()))
        .collect::<Vec<(&str, &str)>>();
    let dependencies = project_config["dependencies"]
        .as_table()
        .unwrap()
        .iter()
        .map(|f| (f.0.as_str(), f.1.as_str().unwrap()))
        .collect::<Vec<(&str, &str)>>();

    println!("package = {:?}\ndependencies = {:?}", package, dependencies);

    let entrypoint_contents = fs::read_to_string(format!("{}/{}", args.root, args.entrypoint))?;
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
