use std::{ops::Deref, rc::Rc, sync::Arc};

use deno_runtime::{
    deno_broadcast_channel::InMemoryBroadcastChannel,
    deno_core::{error::AnyError, FsModuleLoader},
    deno_web::BlobStore,
    worker::WorkerOptions,
    BootstrapOptions,
};

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}
pub struct MainWorkerOptions(WorkerOptions);

impl MainWorkerOptions {
    pub fn into_inner(self) -> WorkerOptions {
        self.0
    }
}

impl Deref for MainWorkerOptions {
    type Target = WorkerOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MainWorkerOptions {
    fn default() -> Self {
        let create_web_worker_cb = Arc::new(|_| {
            panic!("Web workers are not supported in the example");
        });

        let web_worker_preload_module_cb = Arc::new(|_| {
            panic!("Web workers are not supported in the example");
        });

        let bootstrap = BootstrapOptions {
            args: vec![],
            cpu_count: 1,
            debug_flag: false,
            enable_testing_features: false,
            location: None,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: false,
        };

        Self(WorkerOptions {
            bootstrap,
            extensions: vec![],
            unsafely_ignore_certificate_errors: None,
            root_cert_store: None,
            user_agent: "hello_runtime".to_string(),
            seed: None,
            web_worker_preload_module_cb,
            create_web_worker_cb,
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            module_loader: Rc::new(FsModuleLoader),
            get_error_class_fn: Some(&get_error_class_name),
            origin_storage_dir: None,
            blob_store: BlobStore::default(),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
            source_map_getter: None,
            format_js_error_fn: None,
            stdio: Default::default(),
        })
    }
}
