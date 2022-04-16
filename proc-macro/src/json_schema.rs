use anyhow::{anyhow, Result};
use askama::Template;
use heck::{AsPascalCase, AsSnakeCase};
use litrs::Literal;
use proc_macro::{TokenStream, TokenTree};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

pub fn get_string_literal(input: TokenStream) -> Result<String> {
    // let tt = input.into_iter().next().unwrap();
    // match tt {
    //     TokenTree::Literal(v) => Ok(v.to_string()),
    //     _ => Err(anyhow!("Only string literals are allowed")),
    // }
    input
        .into_iter()
        .next()
        .and_then(|v| Literal::try_from(v).ok())
        .and_then(|v| match v {
            Literal::String(s) => Some(s.value().to_string()),
            _ => None,
        })
        .ok_or_else(|| anyhow!("Only string literals are allowed"))
}

#[derive(Template)]
#[template(path = "code.j2")]
pub struct SturctsTemplate {
    structs: Vec<St>,
}

impl SturctsTemplate {
    fn try_new(filename: &str) -> Result<Self> {
        let content = fs::read_to_string(filename)?;
        let schema: Schema = serde_json::from_str(&content)?;
        Ok(Self {
            structs: schema.into_vec_st(),
        })
    }

    pub fn render(filename: &str) -> Result<String> {
        let template = Self::try_new(filename)?;
        Ok(template.render()?)
    }
}

// input
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Schema {
    title: Option<String>,
    #[serde(rename = "type")]
    ty: String,
    properties: Option<HashMap<String, Schema>>,
}

// output
pub struct St {
    name: String,
    fields: Vec<Fd>,
}

pub struct Fd {
    name: String,
    ty: String,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ty: ty.into(),
        }
    }
}

impl Schema {
    pub fn into_vec_st(&self) -> Vec<St> {
        let mut structs = vec![];
        match self.ty.as_str() {
            "object" => {
                let fields: Vec<_> = self
                    .properties
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| process_type(&mut structs, k.as_str(), v))
                    .collect();
                structs.push(St::new(pascal_case(self.title.as_ref().unwrap()), fields));
                structs
            }
            _ => panic!("Not supported yet"),
        }
    }
}

fn process_type(structs: &mut Vec<St>, k: &str, v: &Schema) -> Fd {
    match v.ty.as_str() {
        "object" => {
            let sts = v.into_vec_st();
            structs.extend(sts);
            Fd::new(snake_case(k), gen_name(v.title.as_deref(), k))
        }
        "integer" => Fd::new(snake_case(k), "i64"),
        "float" => Fd::new(snake_case(k), "f64"),
        "string" => Fd::new(snake_case(k), "String"),
        v => panic!("Unsupported schema type: {}", v),
    }
}

fn gen_name(first: Option<&str>, second: &str) -> String {
    pascal_case(first.unwrap_or(second))
}

fn pascal_case(s: &str) -> String {
    AsPascalCase(s).to_string()
}

fn snake_case(s: &str) -> String {
    AsSnakeCase(s).to_string()
}
