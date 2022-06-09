use deno_core::anyhow::Result;
use deno_runtime::{
    deno_core::{self, resolve_url_or_path},
    ops::permissions,
    permissions::Permissions,
    worker::{MainWorker, WorkerOptions},
};
use deno_runtime_live::MainWorkerOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let options = MainWorkerOptions::default();
    // let js_file = format!("{}/examples/fetch.js", env!("CARGO_MANIFEST_DIR"));
    // let js_file = format!("{}/examples/rest.ts", env!("CARGO_MANIFEST_DIR"));
    // let url = resolve_url_or_path(&js_file)?;

    let url = resolve_url_or_path("/tmp/rest.js")?;
    let permissions = Permissions {
        net: Permissions::new_net(&Some(vec![]), false),
        ..Default::default()
    };

    let rt = Builder::new_current_thread()
        .enable_all()
        .max_blocking_threads(32)
        .build()?;

    let fut = async move {
        let mut worker =
            MainWorker::bootstrap_from_options(url.clone(), permissions, options.into_inner());

        worker.execute_main_module(&url).await?;
        worker.run_event_loop(false).await?;
        Ok::<_, AnyError>(())
    };
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, fut)?;
    Ok(())
}
