use deno_core::error::AnyError;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    runtime::start().await
}
