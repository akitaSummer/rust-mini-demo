mod json_schema;

use proc_macro::TokenStream;

use crate::json_schema::{get_string_literal, SturctsTemplate};

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() { println!(\"hello world\"); }".parse().unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let filename = get_string_literal(input).unwrap();
    println!("{}", filename);
    let result = SturctsTemplate::render(&filename).unwrap();
    result.parse().unwrap()
}
