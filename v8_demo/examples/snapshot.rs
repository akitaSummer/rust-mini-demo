use std::fs;

use clap::{Parser, Subcommand};
use v8_demo::JsRuntime;

const SS_FILE: &str = "snapshot.blob";

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Build,
    Run,
}

fn main() {
    JsRuntime::init();
    let args = Args::parse();
    match args.action {
        Action::Build => build_snapshot(SS_FILE),
        Action::Run => run_snapshot(SS_FILE),
    }
}

fn build_snapshot(path: &str) {
    let blob = JsRuntime::create_snapshot();
    fs::write(path, blob).unwrap();
}

fn run_snapshot(path: &str) {
    let blob = fs::read(path).unwrap();
    let mut runtime = JsRuntime::new(Some(blob));
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
    let result = runtime.execute_script(code, false);
    println!("Result is: {:?}", result);
}
