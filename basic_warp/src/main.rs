use std::{collections::HashMap, env};
use warp::Filter;

const ASSET_DIR: &str = "asset";

async fn get_items(
    param: String,
    param_map: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("get {}, {:?}", param, param_map))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let log = warp::log("basic");

    // 宏 不推荐
    let hello = warp::path!("basic" / String / i32)
        .map(|name, age| format!("Hello, {}! Age is {}", name, age));

    // map 同步
    // and_then 异步
    let add = warp::path!("add" / i32 / i32).map(|a, b| format!("a + b = {}", a + b));

    // 推荐
    // /path/$name?$a=$b
    let items = warp::get()
        .and(warp::path("items"))
        .and(warp::path::param::<String>())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(get_items);

    let apis = hello.or(add).or(items).with(log);

    let dir_static = warp::fs::dir(ASSET_DIR);

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", ASSET_DIR)));

    let static_route = dir_static.or(index);

    let routes = static_route.or(apis);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
