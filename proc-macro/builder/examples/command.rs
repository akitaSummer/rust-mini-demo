use builder::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let command = Command::builder()
        .executable("find")
        .args(vec!["-c".into(), "-vvv".into()])
        .env(vec![])
        .current_dir("/User")
        .finish()
        .unwrap();

    println!("{:?}", command)
}

// cargo run --example
