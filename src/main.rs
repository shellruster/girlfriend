mod file;
mod engine;
mod typescript;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() {
        eprintln!("Usage: girlfriend <file>");
        std::process::exit(1);
    }
    let file_path = &args[1];

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(engine::girlfriend(file_path)) {
        eprintln!("error: {error}");
    }
}