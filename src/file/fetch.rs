use deno_core::op;
use deno_core::error::AnyError;

#[op]
pub async fn op_fetch(url: String) -> Result<String, AnyError> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}