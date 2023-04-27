use deno_core::op;
use deno_core::error::AnyError;

#[op]
pub fn op_current_directory(path: String) -> Result<(), AnyError> {
    std::env::set_current_dir(path)?;
    Ok(())
}