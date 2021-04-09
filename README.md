# Bitcoin Wallet Checker

## Summary:
This backend application provides basic functionality for checking the full balance, credit balance, or debit balance of Bitcoin Wallets. It includes a command line interface (CLI) application for interacting with it via the terminal.

Both the backend code and CLI are coded in Rust. Server-side code utilizes a built-in Rusqlite database.

## Project Structure:
The "src" directory contains all server-side code. It is organized into specific files for the main application and three separate code modules. Two of these modules -- db.rs and ingest.rs -- contain functions and code related to fetching from and seeding the database. The third module -- models.rs -- contains structs used as custom types throughout the server-side code.

The "data" directory contains a .csv file of dummy data.

All code related to the command line interface (CLI) can be found in the "client" directory. It contains its own "src" subdirectory, which contains all code for the CLI. "client/src" is orgainzed into the main.rs file and two module files -- fetch.rs and input.rs. Fetch.rs contains helper functions for fetching BTC wallet balances from the server. Input.rs contains functions for getting user input from the terminal.

The server runs on port 8000 by default.

## Available Scripts:
Builds have been already run for the code. The server and command line interface (CLI) need to be run individually.

### `./target/debug/rust_sqlite`
This script runs the latest compiled version of the server. It must be run from the project root directory.

### `./target/debug/client`
This script runs the latest compiled version of the CLI. It must be run from the client directory.

### `cargo build`
This script re-compiles either the server or CLI code. For use with the server, run it in the root directory. For use with the CLI, run it from the client directory.

### `cargo run`
Alternatively, this script can be run to build and run both the server and CLI. For the server, run it from the root directory. To start the CLI, run it from the client directory.



