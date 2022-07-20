use header_handler::{auth, ContextUser};
use serde_json::{json, Value};
use std::{collections::HashMap, convert::Infallible, env, sync::Arc};
use warp::Filter;

mod header_handler;

const ASSET_DIR: &str = "asset";

struct DBPool {}

fn with_pool(
    pool: Arc<DBPool>,
) -> impl Filter<Extract = (Arc<DBPool>,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn rest_create(
    context_user: ContextUser,
    data: Value,
    _db: Arc<DBPool>,
) -> Result<warp::reply::Json, warp::Rejection> {
    Ok(warp::reply::json(&data))
}

async fn rest_list(context_user: ContextUser) -> Result<warp::reply::Json, warp::Rejection> {
    let some_thing = json!([{ "id": 1, "context_user": context_user.id  },{ "id": 2, "context_user": context_user.id  }]);
    Ok(warp::reply::json(&some_thing))
}

async fn rest_get(
    context_user: ContextUser,
    id: i32,
) -> Result<warp::reply::Json, warp::Rejection> {
    let some_thing = json!({ "id": id, "context_user": context_user.id });
    Ok(warp::reply::json(&some_thing))
}

fn rest_api(
    pool: Arc<DBPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let basic_url = warp::path("rest");

    let get = basic_url
        .and(warp::get())
        .and(auth())
        .and(warp::path::param())
        .and_then(rest_get);

    let list = basic_url
        .and(warp::get())
        .and(auth())
        .and(warp::path::end())
        .and_then(rest_list);

    let create = basic_url
        .and(warp::post())
        .and(auth())
        .and(warp::body::json())
        .and(with_pool(pool.clone()))
        .and_then(rest_create);

    get.or(list).or(create)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");

    let log = warp::log("api");

    let db_pool = Arc::new(DBPool {});

    let hello = warp::path!("basic" / String).map(|name| format!("Hello, {}!", name));

    let apis = hello.or(rest_api(db_pool.clone())).with(log);

    let dir_static = warp::fs::dir(ASSET_DIR);

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", ASSET_DIR)));

    let static_route = dir_static.or(index);

    let routes = static_route.or(apis);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
