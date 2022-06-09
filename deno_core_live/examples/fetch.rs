use std::rc::Rc;

use deno_core::{anyhow::Result, FsModuleLoader, JsRuntime, RuntimeOptions};

use deno_core_live::{execute_main_module, ops};

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        extensions: vec![ops::fetch::init()],
        module_loader: Some(Rc::new(FsModuleLoader)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let path = format!("{}/examples/fetch.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, &path).await?;
    Ok(())
}
