mod chunk;
mod chunk_type;
mod png;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        config: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    Decode {
        config: PathBuf,
        chunk_type: String,
    },
    Remove {
        config: PathBuf,
        chunk_type: String,
    },
    Print {
        config: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode {
            config,
            chunk_type,
            message,
            output_file,
        } => {
            println!("encoded: {chunk_type:?} {message:?}")
        }
        Commands::Decode { config, chunk_type } => {
            println!("decoded: {chunk_type:?}")
        }
        Commands::Remove { config, chunk_type } => {
            println!("removed: {chunk_type:?} ")
        }
        Commands::Print { config } => {
            println!("content")
        }
    }
}
