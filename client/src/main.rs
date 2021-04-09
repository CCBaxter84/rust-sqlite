use std::io;
use reqwest::{ Url };
use serde::{ Deserialize };
use serde_json;

#[derive(Deserialize)]
struct Balance {
    balance: f64
}

async fn get_full(address: String) -> Balance {
    let url: String = format!("http://localhost:8000/utxos/{}", address);
    let request = Url::parse(&url).unwrap();
    println!("{}", request);
    let body = reqwest::get(request).await.unwrap().text().await.unwrap();
    let balance: Balance = serde_json::from_str(&body).unwrap();

    return balance;
}

async fn get_spent(address: String, spent: bool) -> Balance {
    let url: String = format!("http://localhost:8000/utxos/{}/{}", address, spent);
    let request = Url::parse(&url).unwrap();
    println!("{}", request);
    let body = reqwest::get(request).await.unwrap().text().await.unwrap();
    let balance: Balance = serde_json::from_str(&body).unwrap();

    return balance;
}

#[tokio::main]
async fn main() {
    println!("What is your BTC Wallet address?");
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap();

    println!("What balance would you like to retrieve?
        1) Full Balance
        2) Spent Balance
        3) Unspent Balance
    ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let decision: i32 = input.trim().parse().unwrap();

    let mut balance: f64 = 0.0;
    if decision == 2 {
        balance = get_spent(address, true).await.balance;
    } else if decision == 3 {
        balance = get_spent(address, false).await.balance;
    } else {
        balance = get_full(address).await.balance;
    }

    //let balance = get_full(address).await;
    println!("{}", balance);
}
