#![feature(proc_macro_hygiene, decl_macro)]

// Import dependencies and initialize modules
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate csv;

pub mod models;
mod db;
mod ingest;

use rocket_contrib::json::Json;
use rusqlite::Connection;
use models::Balance;
use db::{ connect, get_balance };

// @route   GET /uxos/:address
// @desc    Fetch Total BTC Balance for provided wallet address
#[get("/<address>")]
fn fetch_full_balance(address: String) -> Result<Json<Balance>, String> {
    // Connect to DB and query database
    let db_connection = db::connect();
    let statement = match db_connection.prepare("SELECT amount, spent FROM utxos WHERE address = ?;" ) {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into())
    };
    // Return balance
    get_balance(address, statement, "full")
}

// @route   GET /uxos/:address/:spent
// @desc    Fetch BTC Spent or Unspent Balance for provided wallet address
#[get("/<address>/<spent>")]
fn fetch_spent_balance(address: String, spent: String) -> Result<Json<Balance>, String> {
    let db_connection = db::connect();
    let is_spent = if spent == "true" { true } else { false };
    let spent_query = "SELECT amount, spent FROM utxos WHERE address = ? AND spent = true;";
    let unspent_query = "SELECT amount, spent FROM utxos WHERE address = ? AND spent = false;";

    let statement = match db_connection.prepare(if is_spent { spent_query } else { unspent_query} ) {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into())
    };
    // Return balance
    if is_spent {
        return get_balance(address, statement, "spent");
    } else {
        return get_balance(address, statement, "unspent");
    }
}

fn main() {
    // Connect to database, create DB table, and populate data
    let db_connection: Connection = connect();
    db_connection
        .execute(
            "create table if not exists utxos (
                id varchar(64) primary key,
                txid varchar(64) not null,
                address varchar(64) not null,
                amount real not null,
                spent boolean not null
            );",
            []
        )
        .unwrap();
    ingest::run();

    // Start the web server
    rocket::ignite()
        .mount("/utxos", routes![fetch_full_balance, fetch_spent_balance])
        .launch();
}