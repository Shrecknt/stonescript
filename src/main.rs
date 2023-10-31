use clap::Parser;
use std::{fs, path::PathBuf};
use stonescript::{
    ast::parse::{parse, AstNode},
    config::ProjectConfig,
    token::{tokenise, TokenTree},
    VERSION,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Root of the program to compile
    #[arg(short, long, default_value = "./")]
    pub root: PathBuf,
    /// Build directory for the datapack
    #[arg(short, long, default_value = "target")]
    pub target: PathBuf,
    /// Entrypoint file
    #[arg(short, long, default_value = "src/main.ss")]
    pub entrypoint: PathBuf,
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

fn debug_ast(stream: &Vec<AstNode>, indent: usize) {
    for token in stream {
        match token {
            AstNode::Block { contents } => {
                println!("{}Group({{)", " ".repeat(indent));
                debug_ast(contents, indent + 4);
            }
            AstNode::Function {
                function_name,
                arguments,
                return_type,
                contents,
                is_static,
            } => {
                println!(
                    "{}{}Function {}({:?}): {:?}",
                    " ".repeat(indent),
                    if *is_static { "static " } else { "" },
                    function_name,
                    arguments,
                    return_type
                );
                debug_ast(contents, indent + 4);
            }
            _ => {
                println!("{}{:?}", " ".repeat(indent), token);
            }
        }
    }
}

fn main() -> Result<(), eyre::Report> {
    let args = Args::parse();

    println!("Compiling with StoneScript version {}", VERSION);
    println!(
        "{{ root = '{}', target = '{}', entrypoint = '{}' }}",
        args.root.display(),
        args.target.display(),
        args.entrypoint.display()
    );

    let project_config: ProjectConfig =
        toml::from_str(&fs::read_to_string(args.root.join("stonescript.toml"))?)?;

    let target_dir = args.root.join(args.target);
    println!("{{ target_dir = '{}' }}", target_dir.display());

    println!(
        "package = {:?}\ndependencies = {:?}",
        project_config.package, project_config.dependencies
    );

    let entrypoint_contents = fs::read_to_string(args.root.join(args.entrypoint))?;
    let tokenized = tokenise((&mut entrypoint_contents.chars()).into())?;

    println!("Tokens:");
    debug_token_stream(&tokenized, 0);

    println!();

    let mut ast = vec![];
    let mut scope = parse(tokenized, &mut ast)?;
    ast.append(&mut scope);
    println!("AST:");
    debug_ast(&ast, 0);

    Ok(())
}
