use deno_core::op;
use deno_core::error::AnyError;

#[op]
pub fn op_remove_file(path: String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}