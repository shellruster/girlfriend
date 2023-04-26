mod file;
mod engine;
mod typescript;

use clap::{Parser};

/// Javascript shell interpretation
#[derive(Debug, Parser)]
#[command(name = "girlfriend", version)]
#[command(about = "Javascript shell interpretation", long_about = None)]
struct Cli {
    /// File to interpret
    #[clap(value_name = "FILE")]
    file: String,
}

fn main() {
    let args = Cli::parse();
    
    let file_path = args.file;
    
    if !std::path::Path::new(file_path.as_str()).exists() {
        eprintln!("[ERROR]: file does not exist");
        std::process::exit(1);
    }
    
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(engine::girlfriend(file_path.as_str())) {
        eprintln!("error: {error}");
    }
}