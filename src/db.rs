// Dependencies
use rusqlite::{ Connection, Statement, params };
use rocket_contrib::json::Json;
use crate::models::{ Amount, Balance};

// Helper fn for connecting to database & returning connection
pub fn connect() -> Connection {
  let db_connection = Connection::open("utxos.sqlite").unwrap();
  return db_connection;
}

// Helper function for getting BTC balance
pub fn get_balance(address: String, mut statement: Statement) -> Result<Json<Balance>, String> {
    // Run provided query for utxos re: provided BTC address & insert results into an Amount struct
    let results = statement.query_map(params![address], |row| {
        Ok(Amount {
            amount: row.get(0)?
        })
    });

    // Check if results are as expected
    // If yes, calculate and return balance; if no, return an error
    match results {
      Ok(rows) => {
          let collection: rusqlite::Result<Vec<_>> = rows.collect();
          match collection {
              Ok(amounts) => {
                  // Guard clause in case there are no utxos for given wallet address
                  if amounts.len() == 0 {
                      return Err("Wallet not found".into());
                  }
                  // Iterate over utxos and return sum of balance in json format
                  let mut balance = 0.0;
                  for amount in amounts {
                      balance += amount.amount;
                  }
                  return Ok(Json(Balance { balance }))
              },
              Err(e) => return Err(e.to_string())
          }
      }
      Err(_) => Err("Ahh ahh ahh...you didn't say the magic word".into())
    }
}