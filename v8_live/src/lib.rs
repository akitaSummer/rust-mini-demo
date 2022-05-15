mod extensions;
mod state;
mod utils;

use extensions::{Extensions, EXTERNAL_REFERNCES};
use once_cell::sync::OnceCell;
use state::JsRuntimeState;
use utils::execute_script;
use v8::{
    CreateParams, FunctionCodeHandling, HandleScope, Isolate, OwnedIsolate, SnapshotCreator, V8,
};

pub type LocalValue<'a> = v8::Local<'a, v8::Value>;

pub struct JsRuntime {
    isolate: OwnedIsolate,
}

// #[derive(Default)]
// pub struct JsRuntimeParams(CreateParams);

// impl JsRuntimeParams {
//     pub fn new(snapshot: Option<Vec<u8>>) -> Self {
//         let mut params = CreateParams::default();
//         if let Some(snapshot) = snapshot {
//             params = params.snapshot_blob(snapshot);
//         }
//         JsRuntimeParams(params)
//     }

//     pub fn into_inner(self) -> CreateParams {
//         self.0
//     }
// }

impl JsRuntime {
    pub fn init() {
        static V8_INSTANCE: OnceCell<()> = OnceCell::new();
        V8_INSTANCE.get_or_init(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            V8::initialize_platform(platform);
            V8::initialize();
        });
    }

    // pub fn new(params: JsRuntimeParams) -> Self {
    pub fn new(snapshot: Option<Vec<u8>>) -> Self {
        let mut params = CreateParams::default().external_references(&**EXTERNAL_REFERNCES);
        let mut initialized = false;
        if let Some(snapshot) = snapshot {
            params = params.snapshot_blob(snapshot);
            initialized = true;
        }
        let isolate = Isolate::new(params);
        JsRuntime::init_isolate(isolate, initialized)
    }

    pub fn execute_script(
        &mut self,
        code: impl AsRef<str>,
        is_module: bool,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let context = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut HandleScope::with_context(&mut self.isolate, context);
        match execute_script(handle_scope, code, is_module) {
            Ok(value) => Ok(serde_v8::from_v8(handle_scope, value).unwrap()),
            Err(error) => Err(serde_v8::from_v8(handle_scope, error).unwrap()),
        }
    }

    pub fn create_snapshot() -> Vec<u8> {
        let mut sc = SnapshotCreator::new(Some(&EXTERNAL_REFERNCES));
        let isolate = unsafe { sc.get_owned_isolate() };
        let mut runtime = JsRuntime::init_isolate(isolate, false);
        {
            let context = JsRuntimeState::get_context(&mut runtime.isolate);
            let handle_scope = &mut HandleScope::new(&mut runtime.isolate);
            let context = v8::Local::new(handle_scope, context);
            sc.set_default_context(context);
        }

        JsRuntimeState::drop_context(&mut runtime.isolate);
        std::mem::forget(runtime);

        match sc.create_blob(FunctionCodeHandling::Keep) {
            Some(blob) => blob.to_vec(),
            None => panic!("Failed to create snapshot"),
        }
    }

    fn init_isolate(mut isolate: OwnedIsolate, initialized: bool) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        if !initialized {
            let context = JsRuntimeState::get_context(&mut isolate);
            let scope = &mut HandleScope::with_context(&mut isolate, context);
            Extensions::install(scope);
        };
        JsRuntime { isolate }
    }
}
