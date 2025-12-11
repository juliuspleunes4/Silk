use anyhow::Result;
/// Silk CLI - Command-line interface for the Silk compiler
use clap::{Parser, Subcommand};
use silk_compiler::Compiler;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "silk")]
#[command(about = "Silk programming language compiler", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Silk file
    Build {
        /// Input file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Optimization level (0-3)
        #[arg(short = 'O', long, default_value = "0")]
        opt_level: u8,
    },

    /// Compile and run a Silk file
    Run {
        /// Input file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Type-check a Silk file without compiling
    Check {
        /// Input file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Show tokens (lexer output) for debugging
    Lex {
        /// Input file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let compiler = Compiler::new();

    match cli.command {
        Commands::Build {
            file,
            output: _,
            opt_level,
        } => {
            println!("Building {} (opt-level: {})...", file.display(), opt_level);
            let source = fs::read_to_string(&file)?;

            // For now, just lex
            match compiler.lex(&source) {
                Ok(tokens) => {
                    println!("✓ Lexing successful ({} tokens)", tokens.len());
                    // TODO: Continue with parsing, semantic analysis, codegen
                    println!("⚠ Full compilation not yet implemented");
                }
                Err(e) => {
                    eprintln!("✗ Lexing failed: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Run { file } => {
            println!("Running {}...", file.display());
            println!("⚠ Run command not yet implemented");
        }

        Commands::Check { file } => {
            println!("Type-checking {}...", file.display());
            let source = fs::read_to_string(&file)?;

            match compiler.lex(&source) {
                Ok(_) => {
                    println!("✓ Lexing successful");
                    println!("⚠ Type checking not yet implemented");
                }
                Err(e) => {
                    eprintln!("✗ Lexing failed: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Lex { file } => {
            let source = fs::read_to_string(&file)?;

            match compiler.lex(&source) {
                Ok(tokens) => {
                    println!("Tokens for {}:\n", file.display());
                    for (i, token) in tokens.iter().enumerate() {
                        println!("{:4}: {:?}", i, token);
                    }
                    println!("\nTotal: {} tokens", tokens.len());
                }
                Err(e) => {
                    eprintln!("✗ Lexing failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
