use v8_live::JsRuntime;

fn main() {
    JsRuntime::init();
    let mut runtime = JsRuntime::new(None);

    let code = r#"
        function hello_world() {
            print("Hello Rust");
            // return {
            //     status: 200,
            //     message: "Hello World"
            // };
            return fetch("https://www.rust-lang.org/")
        }
        hello_world();
    "#;

    let result = runtime.execute_script(code);
    println!("Result is: {:#?}", result);
}
