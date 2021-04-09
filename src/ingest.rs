// Dependencies
use csv;
use std::error::Error;
use rusqlite::{Connection, named_params};
use crate::models::Utxo;

// Helper fn for inserting data from .csv into database
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
  // Connect to database and .csv containing dummy data
  let db_connection = Connection::open("utxos.sqlite").unwrap();
  let mut reader = csv::Reader::from_path(path)?;

  // Iterate over dummy records, format for DB insertion, and insert each into DB
  for result in reader.records() {
    let record = result?;
    let amount = record[3].parse::<f64>().unwrap();
    let spent = if &record[4] == "TRUE" { true } else { false };
    let format_record = Utxo {
      id: record[0].into(),
      txid: record[1].into(),
      address: record[2].into(),
      amount: amount,
      spent: spent
    };

    let mut statement = match db_connection.prepare("insert into utxos (id, txid, address, amount, spent) values ($1, $2, $3, $4, $5);") {
      Ok(statement) => statement,
      Err(e) => return Err(e.into())
    };

    let _results = statement.execute(
      named_params!{"$1": format_record.id, "$2": format_record.txid, "$3": format_record.address, "$4": format_record.amount, "$5": format_record.spent });
  }

  // Return empty ok if it works
  Ok(())
}

pub fn run() {
  // Run the insertion fn above and throw an error if it is unsuccessful
  if let Err(e) = read_from_file("./data/btc_utxos.csv") {
    eprintln!("{}", e);
  }
}