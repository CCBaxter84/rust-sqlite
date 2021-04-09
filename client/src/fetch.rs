use reqwest::{ Url };
use serde::{ Deserialize };
use serde_json;
use std::future::Future;

#[derive(Deserialize)]
pub struct Balance {
    pub balance: f64
}

async fn check_result(url: String) -> Result<Balance, String> {
  let request = Url::parse(&url).unwrap();
  let result = reqwest::get(request).await;
  match result {
    Ok(res) => {
      let body = res.text().await.unwrap();
      let balance = serde_json::from_str(&body);
      match balance {
        Ok(res) => Ok(res),
        Err(_) => Err("No matching wallet".into())
      }
    },
    Err(e) => return Err(e.to_string())
  }
}

pub async fn get_full(address: String) -> Result<Balance, String> {
  let url: String = format!("http://localhost:8000/utxos/{}", address);
  check_result(url).await
}

pub async fn get_spent(address: String, spent: bool) -> Result<Balance, String> {
  let url: String = format!("http://localhost:8000/utxos/{}/{}", address, spent);
  check_result(url).await
}