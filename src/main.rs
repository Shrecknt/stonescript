use clap::Parser;
use std::{
    fs::{self, File},
    path::PathBuf,
};
use stonescript::{
    config::ProjectConfig,
    token::parse_from_reader,
    VERSION, ast::Statement, TokenIter
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

    let entrypoint_file = File::open(args.root.join(args.entrypoint))?;
    let tokenized = parse_from_reader(entrypoint_file)?;

    //println!("Tokens:\n{:#?}", tokenized);
    let statements: Vec<Statement> = TokenIter::from(&tokenized).parse()?;
    println!("{:?}", statements);

    // let mut ast = vec![];
    // let mut scope = TokenStream::new(tokenized).parse(&mut ast)?;
    // ast.append(&mut scope);
    // println!("AST:");
    // debug_ast(&ast, 0);

    Ok(())
}
