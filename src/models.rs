// Dependencies
use serde::{ Serialize, Deserialize };

#[derive(Serialize)]
pub struct Amount {
    pub amount: f64
}

#[derive(Serialize)]
pub struct Balance {
  pub balance: f64
}

#[derive(Debug, Deserialize)]
pub struct Utxo {
  pub id: String,
  pub txid: String,
  pub address: String,
  pub amount: f64,
  pub spent: bool
}