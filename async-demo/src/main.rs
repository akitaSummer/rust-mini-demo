use futures::future::join_all;

// async fn fetch_path(path: String) -> Result<String, reqwest::Error> {
async fn fetch_path(path: String) -> surf::Result<String> {
    let mut result: String = String::new();
    // match reqwest::get(&path).await {
    match surf::get(&path).await {
        Ok(mut res) => {
            // match res.text().await {
            match res.body_string().await {
                Ok(text) => {
                    result = format!("path is {:?}, text length is {:?}", &path, text.len());
                },
                Err(_) => { println!("res.text error") }
            }
        },
        Err(_) => { println!("path error"); }
    }
    Ok(result)
}

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {

#[async_std::main]
async fn main() -> surf::Result<()> {
    let paths = vec![
        "https://crates.io/".to_string(),
        "https://juejin.cn/".to_string()
    ];

    let result = join_all(paths.into_iter().map(|path| {
        fetch_path(path)
    })).await;

    let mut results_list: Vec<String> = vec![];

    for elem in result {
        if elem.is_ok() {
            results_list.push(elem.unwrap());
        } else {
            return Err(elem.unwrap_err());
        }
    }

    println!("{:?}", &results_list);

    Ok(())
}
