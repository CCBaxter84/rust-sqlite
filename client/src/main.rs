extern crate settimeout;

mod fetch;
mod input;

use std::io;
use std::time::Duration;
use settimeout::set_timeout;
use fetch::{ get_full, get_spent };
use input::{ get_wallet, get_request_type, get_final_input, get_display };

#[tokio::main]
async fn main() {
    // Ask user for balance until they elect to quit
    let mut done = false;
    while done == false {
        // Get user's BTC wallet address
        println!("What is your BTC Wallet address?");
        let address: String = get_wallet();

        // Get balance type to query
        println!("What balance would you like to retrieve?
            1) Full Balance
            2) Spent Balance
            3) Unspent Balance
        ");
        let decision: i32 = get_request_type();

        // Get requested balance
        // Throw error message if BTC wallet is invalid
        let mut balance: f64 = 0.0;
        if decision == 2 {
            let res = get_spent(address, true).await;
            match res {
                Ok(sum) => {
                    balance += sum.balance;
                },
                Err(e) => {
                    println!("{}", e);
                    set_timeout(Duration::from_secs(2)).await;
                    continue;
                }
            }
        } else if decision == 3 {
            let res = get_spent(address, false).await;
            match res {
                Ok(sum) => {
                    balance += sum.balance;
                },
                Err(e) => {
                    println!("{}", e);
                    set_timeout(Duration::from_secs(2)).await;
                    continue;
                }
            }
        } else {
            let res = get_full(address).await;
            match res {
                Ok(sum) => {
                    balance += sum.balance;
                },
                Err(e) => {
                    println!("{}", e);
                    set_timeout(Duration::from_secs(2)).await;
                    continue;
                }
            }
        }
        // Print Formatted Wallet balance
        let display = get_display(decision);
        println!("{} BTC Wallet Balance: {}", display, balance);

        // Ask user if they want to perform another tx
        // If yes, repeat loop; if no, quit program
        set_timeout(Duration::from_secs(2)).await;
        println!("Would you like to check another balance?
        'Y' or 'N'");
        let result: String = get_final_input();
        done = if result == "y" || result == "yes" { false } else { true };
    }
}

