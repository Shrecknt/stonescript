use clap::Parser;
use std::{
    fs::{self, File},
    path::PathBuf,
};
use stonescript::{
    config::ProjectConfig,
    hir::{Statement, ToTokens},
    mir::{Mangle, Scope, ToMir},
    token::parse_from_reader,
    TokenIter, VERSION,
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

fn main() -> eyre::Result<()> {
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

    let statements: Vec<Statement> = TokenIter::from(&tokenized).parse()?;
    println!("\nAST:\n\n{:#?}", statements.clone().into_tokens());

    let mir_unmangled = statements.into_mir();
    println!("MIR (pre-mangled): {:?}", mir_unmangled);

    let mir_mangled = mir_unmangled.mangle(&mut Scope::new(&project_config.package.name));
    println!("\nMIR (mangled): {:?}", mir_mangled);

    // let rebuilt_statements: Vec<RebuiltStatement> = rebuild_from_ast(statements, &project_config);
    // println!("\nRebuilt AST:\n\n{:#?}", rebuilt_statements);

    Ok(())
}
