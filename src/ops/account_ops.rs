use diesel::prelude::*;
use std::io;

use crate::db::establish_connection;
use crate::models::{Account, NewAccount};

pub fn add_account() {
    let connection = &mut establish_connection();
    use crate::schema::accounts::dsl::*;

    println!("Adding a new account...");
    println!("Please enter a name: ");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).expect("Failed.");
    let name_input = name_input.trim();

    println!("What kind of account is it?: ");
    let mut type_of_input = String::new();
    io::stdin().read_line(&mut type_of_input).expect("Failed.");
    let type_of_input = type_of_input.trim();

    println!("What is the current balance of the account?: ");
    let mut starting_balance_input = String::new();
    io::stdin()
        .read_line(&mut starting_balance_input)
        .expect("Failed.");

    let starting_balance_input: i32 = starting_balance_input
        .trim()
        .parse()
        .expect("Please type a number!");

    let new_account = NewAccount {
        name: name_input,
        type_of: type_of_input,
        starting_balance: starting_balance_input,
        current_balance: starting_balance_input,
    };

    diesel::insert_into(accounts)
        .values(&new_account)
        .execute(connection)
        .expect("Something went wrong.");
}

pub fn update_account() {
    use crate::schema::accounts::dsl::*;
    let connection = &mut establish_connection();

    println!("What account would you like to update?");

    display_accounts();

    let mut request = String::new();
    io::stdin().read_line(&mut request).expect("Failed!");
    let request: i32 = request.trim().parse().expect("Please enter a number!");

    let account_to_update = accounts
        .find(request)
        .select(Account::as_select())
        .first(connection)
        .optional();

    match account_to_update {
        Ok(Some(account_to_update)) => {
            println!("Was the transaction income or an expense?");
            println!("1) Income.");
            println!("2) Expense.");
            let mut transaction_type = String::new();
            io::stdin()
                .read_line(&mut transaction_type)
                .expect("Failed.");
            let transaction_type: i32 = transaction_type
                .trim()
                .parse()
                .expect("Please type a number!");

            println!("How much was the transaction?");
            let mut amount = String::new();
            io::stdin().read_line(&mut amount).expect("Failed.");
            let amount: i32 = amount.trim().parse().expect("Please type a number!");

            let new_balance: i32;

            match transaction_type {
                1 => new_balance = account_to_update.current_balance + amount,
                2 => new_balance = account_to_update.current_balance - amount,
                _ => new_balance = account_to_update.current_balance,
            }

            let update_account = Account {
                id: account_to_update.id,
                name: account_to_update.name,
                type_of: account_to_update.type_of,
                starting_balance: account_to_update.starting_balance,
                current_balance: new_balance,
            };

            diesel::update(accounts.find(request))
                .set(&update_account)
                .execute(connection)
                .expect("Something went wrong");
        }
        Ok(None) => println!("Unable to find account {}", request),
        Err(_) => println!("An error occured while fetching account {}", request),
    }
}

pub fn display_accounts() {
    println!("Showing all accounts...");
    use crate::schema::accounts::dsl::*;

    let connection = &mut establish_connection();

    let results = accounts
        .filter(name.is_not_null())
        .select(Account::as_select())
        .load(connection)
        .expect("Something went wrong.");

    println!("Displaying {} accounts", results.len());
    for account in results {
        println!("{:?}", account);
    }
}
