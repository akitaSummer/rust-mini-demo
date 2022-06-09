use deno_core::{anyhow::Result, JsRuntime, RuntimeOptions};
use deno_core_live::eval;

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions::default();
    let mut rt = JsRuntime::new(options);
    let code = include_str!("basic.js");
    let result: String = eval(&mut rt, code).await?;
    print!("Rust: {:?}", result);
    Ok(())
}
