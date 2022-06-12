use deno_core_demo::execute_main_module;
use lazy_static::lazy_static;
use std::rc::Rc;

lazy_static! {
    static ref SNAPSHOT: &'static [u8] = {
        let data = include_bytes!("../snapshots/main.bin");
        let decompressed = zstd::decode_all(&data[..]).unwrap().into_boxed_slice();
        Box::leak(decompressed)
    };
}

use deno_core::{anyhow::Result, FsModuleLoader, JsRuntime, RuntimeOptions, Snapshot};

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        startup_snapshot: Some(Snapshot::Static(&*SNAPSHOT)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let path = format!("{}/examples/module.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, &path).await?;
    Ok(())
}
