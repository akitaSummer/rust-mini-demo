use core::blockchain;
use std::thread;
use std::time::Duration;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    println!("start mining ...");
    thread::sleep(Duration::from_secs(5));
    bc.add_block("a -> b: 5btc".to_string());
    println!("produce a block !");

    println!("start mining ...");
    thread::sleep(Duration::from_secs(5));
    bc.add_block(String::from("c -> d: 1btc"));
    println!("produce a block !");

    for b in bc.blocks {
        println!("++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
