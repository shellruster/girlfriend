use deno_core::op;
use deno_core::error::AnyError;

#[op]
pub fn op_make_directory(path: String) -> Result<(), AnyError> {
    std::fs::create_dir(path)?;
    Ok(())
}