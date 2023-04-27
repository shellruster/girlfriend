use deno_core::op;
use deno_core::error::AnyError;

#[op]
pub fn op_list_directory(path: String) -> Result<Vec<String>, AnyError> {
    let result = std::fs::read_dir(path)?
        .map(|entry| entry.map(|e| e.path().display().to_string()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    Ok(result)
}