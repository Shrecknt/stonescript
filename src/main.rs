use crate::{
    config::ProjectConfig,
    token::{tokenise, TokenTree},
};
use clap::Parser;
use std::{fs, path::PathBuf};
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
    UnexpectedToken(String, &'static str),
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

fn debug_token_stream(stream: &Vec<TokenTree>, indent: usize) {
    for token in stream {
        match token {
            TokenTree::Group(group) => {
                println!("{}Group({:?})", " ".repeat(indent), group.delimiter);
                debug_token_stream(&group.tokens, indent + 4)
            }
            TokenTree::Ident(ident) => {
                println!("{}Ident({:?})", " ".repeat(indent), ident.token);
            }
            TokenTree::Literal(literal) => {
                println!("{}Literal({:?})", " ".repeat(indent), literal.value);
            }
            TokenTree::Punct(punct) => {
                println!("{}Punct({:?})", " ".repeat(indent), punct.token);
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

    println!(
        "package = {:?}\ndependencies = {:?}",
        project_config.package, project_config.dependencies
    );

    let entrypoint_contents = fs::read_to_string(args.root.join(args.entrypoint))?;
    let tokenized = tokenise((&mut entrypoint_contents.chars()).into())?;

    // println!("Tokenized: {:?}", tokenized);

    println!("Tokens:\n");
    debug_token_stream(&tokenized, 0);

    Ok(())
}
