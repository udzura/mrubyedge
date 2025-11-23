use clap::{Parser, Subcommand};

mod subcommands;

#[derive(Parser)]
#[command(name = "mrbedge")]
#[command(about = "mruby/edge command line interface", long_about = None)]
#[command(version)]
#[command(args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    run_args: Option<subcommands::run::RunArgs>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Ruby code or binary.
    /// Run is invoked when rb/mrb file is directly passed to the command
    Run(subcommands::run::RunArgs),
    /// Generate WebAssembly binary from Ruby code
    Wasm,
    /// Compile Ruby script to mrb
    CompileMrb(subcommands::compile_mrb::CompileMrbArgs),
    /// Scaffold the package project with a wasm binary
    Scaffold {
        #[command(subcommand)]
        scaffold_type: ScaffoldType,
    },
}

#[derive(Subcommand)]
enum ScaffoldType {
    /// Scaffold npm package
    Npm,
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run(args)) => {
            subcommands::run::execute(args)?;
        }
        Some(Commands::Wasm) => {
            subcommands::wasm::execute();
        }
        Some(Commands::CompileMrb(args)) => {
            subcommands::compile_mrb::execute(args)?;
        }
        Some(Commands::Scaffold { scaffold_type }) => match scaffold_type {
            ScaffoldType::Npm => {
                subcommands::scaffold::execute_npm();
            }
        },
        None => {
            if let Some(args) = cli.run_args {
                subcommands::run::execute(args)?;
            } else {
                eprintln!("No subcommand was used. Use --help for more information.");
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
