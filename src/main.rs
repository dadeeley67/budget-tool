#[macro_use]
extern crate diesel;
extern crate dotenvy;

mod db;
mod models;
mod ops;
mod schema;

use std::io;

use ops::account_ops::{add_account, display_accounts, update_account};
use ops::transaction_ops::{display_transactions, execute_transaction, update_transaction};

fn main() {
    loop {
        println!("What would you like to do?");
        println!("1) Add an account.");
        println!("2) Execute a transaction.");
        println!("3) Display all accounts.");
        println!("4) Display all transactions.");
        println!("5) Update account.");
        println!("6) Update transaction.");
        println!("0) Quit.");

        let mut request = String::new();
        io::stdin().read_line(&mut request).expect("Failed!");
        let request: u32 = request.trim().parse().expect("Please enter a number!");

        match request {
            1 => add_account(),
            2 => execute_transaction(),
            3 => display_accounts(),
            4 => display_transactions(),
            5 => update_account(),
            6 => update_transaction(),
            _ => break,
        }
    }
}
