use futures::future::{join, join_all};
use async_std::task::sleep;
use std::time;
use std::sync::Arc;
use std::sync::Mutex;

async fn hello() {
    println!("hello world");
}

async fn connect_db() -> String {
    sleep(time::Duration::from_secs(1)).await;
    String::from("client")
}

async fn open_file() -> String {
    sleep(time::Duration::from_secs(2)).await;
    String::from("file")
}

async fn build_city(cities: Vec<&str>) -> Vec<String> {
    let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    join_all(cities.into_iter().map(|city| {
        build(results.clone(), city)
    })).await;
   return results.lock().unwrap().clone();
}

async fn build(results: Arc<Mutex<Vec<String>>>, city: &str) {
    sleep(time::Duration::from_secs(2)).await;
    results.lock().unwrap().push(format!("super city {}", city));
}

#[async_std::main]
async fn main() {
    let now = time::Instant::now();
    hello().await;
    let (db, file) = join(connect_db(), open_file()).await;
    println!("{:?}, {:?}", db, file);
    let cities = vec!["beijing", "shanghai", "guangzhou"];
    let results = build_city(cities).await;
    println!("{:?}", results);
    println!("executed in {:?}", now.elapsed());
}
