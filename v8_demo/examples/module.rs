use v8_demo::JsRuntime;

fn main() {
    JsRuntime::init();
    let mut runtime = JsRuntime::new(None);

    let code = r#"
        async function hello_world() {
            print("Hello Rust");
            // return {
            //     status: 200,
            //     message: "Hello World"
            // };
            return await fetch("https://www.rust-lang.org/")
        }
        let result = await hello_world();
        print(result);
    "#;

    let result = runtime.execute_script(code, true);
    println!("Result is: {:#?}", result);
}
