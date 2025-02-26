use anyhow::Result;
use clap::{Parser, Subcommand};
use ntk_compression::{compression::Compressor, decompression::Decompressor};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        input: String,
        output: String,
    },
    Decompress {
        input: String,
        output: String,
    },
}

fn print_file_info(path: &str) -> Result<()> {
    let metadata = std::fs::metadata(path)?;
    println!("Fichier '{}' : {} octets", path, metadata.len());
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { input, output } => {
            print_file_info(&input)?;
            let mut compressor = Compressor::new();
            let stats = compressor.compress_file(&input, &output)?;
            stats.print();
            print_file_info(&output)?;
        }
        Commands::Decompress { input, output } => {
            print_file_info(&input)?;
            let mut decompressor = Decompressor::new();
            let stats = decompressor.decompress_file(&input, &output)?;
            stats.print();
            print_file_info(&output)?;
        }
    }

    Ok(())
}

