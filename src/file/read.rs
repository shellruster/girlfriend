use deno_core::op;
use deno_core::error::AnyError;

#[op]
pub async fn op_read_file(path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}