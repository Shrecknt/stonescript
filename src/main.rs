use std::{fs, path::PathBuf};
use clap::Parser;
use crate::{config::ProjectConfig, token::{tokenise, TokenTree}};
use thiserror::Error;

mod config;
mod stream;
mod token;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    #[error("Unexpected {0:?} while parsing {1}")]
    UnexpectedToken(String, &'static str)
}

pub type ParseResult<T> = Result<T, ParseError>;

pub(crate) trait ExpectChar {
    fn expect_char(self) -> ParseResult<char>;
}

impl ExpectChar for Option<char> {
    fn expect_char(self) -> ParseResult<char> {
        if let Some(char) = self {
            Ok(char)
        } else {
            Err(ParseError::UnexpectedEOF)
        }
    }
}

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

fn debug_token_stream(stream: &Vec<TokenTree>) {
    for token in stream {
        match token {
            TokenTree::Group(group) => {
                println!("{:?}", group);
            }
            TokenTree::Ident(ident) => {
                println!("{:?}", ident);
            }
            TokenTree::Literal(literal) => {
                println!("{:?}", literal);
            }
            TokenTree::Punct(punct) => {
                println!("{:?}", punct);
            }
        }
    }
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
    let tokenized = tokenise((&entrypoint_contents).into())?;

    // println!("Tokenized: {:?}", tokenized);

    println!("Tokens:");
    debug_token_stream(&tokenized);

    Ok(())
}
