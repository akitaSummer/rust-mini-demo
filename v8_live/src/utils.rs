use crate::LocalValue;
use v8::{HandleScope, Script, TryCatch};

pub fn execute_script<'a>(
    scope: &mut HandleScope<'a>,
    code: impl AsRef<str>,
) -> Result<LocalValue<'a>, LocalValue<'a>> {
    let scope = &mut TryCatch::new(scope);
    let source = v8::String::new(scope, code.as_ref()).unwrap();
    Script::compile(scope, source, None)
        .and_then(|script| script.run(scope))
        .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
}
