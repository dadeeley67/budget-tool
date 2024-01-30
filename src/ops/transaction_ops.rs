use diesel::prelude::*;
use std::io;

use crate::db::establish_connection;
use crate::models::{NewTransaction, Transaction};

pub fn execute_transaction() {
    let connection = &mut establish_connection();
    use crate::schema::transactions::dsl::*;

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

    println!("Please enter a name: ");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).expect("Failed.");
    let name_input = name_input.trim();

    println!("Anything you wanna write down about it?: ");
    let mut notes_input = String::new();
    io::stdin().read_line(&mut notes_input).expect("Failed.");
    let notes_input = notes_input.trim();

    let mut payee_input = String::new();
    let mut monetary_input = String::new();
    let new_transaction: NewTransaction;

    if transaction_type.eq(&1) {
        println!("Who paid ya?: ");
        io::stdin().read_line(&mut payee_input).expect("Failed.");
        let payee_input = payee_input.trim();

        println!("How much did they pay ya?: ");

        io::stdin().read_line(&mut monetary_input).expect("Failed.");
        let monetary_input: i32 = monetary_input
            .trim()
            .parse()
            .expect("Please type a number!");

        new_transaction = NewTransaction {
            name: name_input,
            payee: &payee_input,
            inflow: monetary_input,
            outflow: monetary_input * -1,
            notes: notes_input,
        };
    } else {
        println!("Who'd ya pay?: ");

        io::stdin().read_line(&mut payee_input).expect("Failed.");
        let payee_input = payee_input.trim();

        println!("How much did ya pay 'em?: ");

        io::stdin().read_line(&mut monetary_input).expect("Failed.");
        let monetary_input: i32 = monetary_input
            .trim()
            .parse()
            .expect("Please type a number!");

        new_transaction = NewTransaction {
            name: name_input,
            payee: &payee_input,
            inflow: monetary_input * -1,
            outflow: monetary_input,
            notes: notes_input,
        };
    }

    diesel::insert_into(transactions)
        .values(&new_transaction)
        .execute(connection)
        .expect("Something went wrong.");
}

pub fn display_transactions() {
    println!("Showing all transactions...");
    use crate::schema::transactions::dsl::*;

    let connection = &mut establish_connection();

    let results = transactions
        .filter(name.is_not_null())
        .select(Transaction::as_select())
        .load(connection)
        .expect("Something went wrong.");

    println!("Displaying {} transactions", results.len());
    for transaction in results {
        println!("{:?}", transaction);
    }
}

pub fn update_transaction() {
    use crate::schema::transactions::dsl::*;
    let connection = &mut establish_connection();

    println!("What transaction would you like to update?");

    display_transactions();

    let mut request = String::new();
    io::stdin().read_line(&mut request).expect("Failed!");
    let request: i32 = request.trim().parse().expect("Please enter a number!");

    let transaction_to_update = transactions
        .find(request)
        .select(Transaction::as_select())
        .first(connection)
        .optional();

    match transaction_to_update {
        Ok(Some(transaction_to_update)) => {
            println!("Was the transaction income or an expense?");
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

            let new_inflow: i32;
            let new_outflow: i32;

            match transaction_type {
                1 => {
                    new_inflow = amount;
                    new_outflow = amount * -1;
                }
                2 => {
                    new_inflow = amount * -1;
                    new_outflow = amount;
                }
                _ => {
                    new_inflow = transaction_to_update.inflow;
                    new_outflow = transaction_to_update.outflow
                }
            }
            let update_transaction = Transaction {
                id: transaction_to_update.id,
                name: transaction_to_update.name,
                payee: transaction_to_update.payee,
                inflow: new_inflow,
                outflow: new_outflow,
                notes: transaction_to_update.notes,
            };

            diesel::update(transactions.find(request))
                .set(&update_transaction)
                .execute(connection)
                .expect("Something went wrong");
        }
        Ok(None) => println!("Unable to find account {}", request),
        Err(_) => println!("An error occured while fetching transaction {}", request),
    }
}
