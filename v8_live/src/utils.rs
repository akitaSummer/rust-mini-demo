use crate::LocalValue;
use v8::{script_compiler, HandleScope, Promise, Script, ScriptOrigin, TryCatch};

pub fn execute_script<'a>(
    scope: &mut HandleScope<'a>,
    code: impl AsRef<str>,
    is_module: bool,
) -> Result<LocalValue<'a>, LocalValue<'a>> {
    let scope = &mut TryCatch::new(scope);
    let source = v8::String::new(scope, code.as_ref()).unwrap();
    let origin = create_origin(scope, "dummy.js", is_module);
    if is_module {
        let source = script_compiler::Source::new(source, Some(&origin));
        let module = script_compiler::compile_module(scope, source).unwrap();
        module.instantiate_module(scope, module_callback).unwrap();
        let result = module.evaluate(scope).unwrap();
        let promise = v8::Local::<v8::Promise>::try_from(result).unwrap();
        match promise.state() {
            v8::PromiseState::Pending => panic!("We don't know hot to process pending promise"),
            v8::PromiseState::Fulfilled => Ok(promise.result(scope)),
            v8::PromiseState::Rejected => Err(promise.result(scope)),
        }
    } else {
        Script::compile(scope, source, Some(&origin))
            .and_then(|script| script.run(scope))
            .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
    }
}

fn module_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    name: v8::Local<'a, v8::String>,
    arr: v8::Local<'a, v8::FixedArray>,
    module: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    println!(
        "context: {:?}, name: {:?}, arr: {:?}, module: {:?}",
        context, name, arr, module
    );
    Some(module)
}

fn create_origin<'a>(
    scope: &mut HandleScope<'a>,
    filename: impl AsRef<str>,
    is_module: bool,
) -> ScriptOrigin<'a> {
    let name: LocalValue = v8::String::new(scope, filename.as_ref()).unwrap().into();
    ScriptOrigin::new(
        scope,
        name.clone(),
        0,
        0,
        false,
        0,
        name,
        false,
        false,
        is_module,
    )
}
