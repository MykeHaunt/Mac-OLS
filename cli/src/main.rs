use clap::{Parser, Subcommand};
use ecutil_core::binary::BinaryFile;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Load { file: String },
    Detect { file: String },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Load { file } => {
            let bin_file = BinaryFile::load(file).unwrap();
            println!("Loaded {} bytes", bin_file.data.len());
        }
        Commands::Detect { file } => {
            // Map detection implementation
        }
    }
}
