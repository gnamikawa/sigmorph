use crate::compiler;
use clap::{Parser as CliParser, Subcommand};
use std::path;

// #[command(Genzo Namikawa, 0.0.1, A highly opinionated general-purpose programming language.)]
#[derive(CliParser)]
struct Args {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compiles project to binary
    Build { path: String },
}

pub fn parse_cli_arguments() {
    let args = Args::parse();

    match &args.commands {
        Some(Commands::Build { path: path_str }) => {
            let path = path::Path::new(path_str);

            if !path_str.is_empty() && path.is_file() {
                println!("Building project at '{}'", path_str);
                compiler::compile(path_str).unwrap_or_default();
            } else {
                println!("Invalid path was specified. '{}'", path_str);
            }
        }
        None => {
            println!("No arguments specified. Try --help or -h for available commands.");
        }
    }
}
