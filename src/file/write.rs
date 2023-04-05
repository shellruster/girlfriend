use deno_core::op;
use deno_core::error::AnyError;

#[op]
pub async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}