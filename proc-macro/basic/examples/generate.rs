pub mod generated {
    use proc_macro::generate;
    generate!("./persion.json");
}

use generated::*;

fn main() {}
