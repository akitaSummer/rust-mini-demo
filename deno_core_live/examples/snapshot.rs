use lazy_static::lazy_static;
use std::rc::Rc;

lazy_static! {
    static ref SNAPSHOT: &'static [u8] = {
        let data = include_bytes!("../snapshots/main.bin");
        let decompressed = zstd::decode_all(&data[..]).unwrap().into_boxed_slice();
        Box::leak(decompressed)
    };
}

use deno_core::{
    anyhow::Result, resolve_url_or_path, serde::de::DeserializeOwned, serde_v8, v8, FsModuleLoader,
    JsRuntime, RuntimeOptions, Snapshot,
};

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

async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let ret = rt.execute_script("hello_world", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result).unwrap())
}

async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>) -> Result<()> {
    let url = resolve_url_or_path(path.as_ref())?;
    let id = rt.load_main_module(&url, None).await?;
    let mut receiver = rt.mod_evaluate(id);
    tokio::select! {
        resolved = &mut receiver => {
            resolved.expect("failed to evaluate module")
        },
        _ = rt.run_event_loop(false) => {
            receiver.await.expect("failed to evaluate module")
        }
    }
}
