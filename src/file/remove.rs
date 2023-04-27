use deno_core::op;
use deno_core::error::AnyError;

#[op]
pub fn op_remove(path: String) -> Result<(), AnyError> {
    let path = std::path::Path::new(&path);

    if path.is_file() {
        std::fs::remove_file(path)?;
    } else if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    }

    Ok(())
}