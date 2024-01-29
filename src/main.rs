use chrono::{self, DateTime, Local};
use std::io;

#[derive(Clone)]
enum Transaction {
    Income {
        name: String,
        payee: String,
        inflow: u32,
        notes: String,
        date: DateTime<Local>,
    },
    Expense {
        name: String,
        payee: String,
        inflow: i32,
        outflow: i32,
        notes: String,
        date: DateTime<Local>,
    },
}

#[derive(Clone)]
struct Asset {
    name: String,
    type_of: String,
    starting_balance: u32,
    current_balance: i32,
    transactions: Vec<Transaction>,
}

struct Liability {
    name: String,
    type_of: String,
    starting_balance: i32,
    current_balance: i32,
    interest_rate: u32,
    transactions: Vec<Transaction>,
}

fn main() {
    let mut assets: Vec<Asset> = Vec::new();
    let mut liabilities: Vec<Liability> = Vec::new();

    loop {
        println!("What would you like to do?");
        println!("1) Add an asset.");
        println!("2) Add a liability.");
        println!("3) Execute a transaction.");
        println!("4) Display all assets.");
        println!("5) Display all liabilities.");
        println!("6) Quit.");

        let mut request = String::new();
        io::stdin().read_line(&mut request).expect("Failed!");
        let request: u32 = request.trim().parse().expect("Please enter a number!");

        match request {
            1 => add_asset(&mut assets),
            2 => add_liability(&mut liabilities),
            3 => execute_transaction(&mut assets),
            4 => display_assets(&assets),
            5 => display_liabilities(&liabilities),
            _ => break,
        }
    }
}

fn add_asset(assets: &mut Vec<Asset>) {
    println!("Adding a new asset...");
    println!("Please enter a name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed.");
    let name = name.trim();

    println!("What kind of account is it?: ");
    let mut type_of = String::new();
    io::stdin().read_line(&mut type_of).expect("Failed.");
    let type_of = type_of.trim();

    println!("What is the current balance of the account?: ");
    let mut starting_balance = String::new();
    io::stdin()
        .read_line(&mut starting_balance)
        .expect("Failed.");

    let starting_balance: u32 = starting_balance
        .trim()
        .parse()
        .expect("Please type a number!");

    assets.push(Asset {
        name: name.to_string(),
        type_of: type_of.to_string(),
        starting_balance,
        current_balance: starting_balance as i32,
        transactions: Vec::new(),
    });
}

fn add_liability(liabilities: &mut Vec<Liability>) {
    println!("Adding a new liability...");
    println!("Please enter a name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed.");
    let name = name.trim();

    println!("What kind of account is it?: ");
    let mut type_of = String::new();
    io::stdin().read_line(&mut type_of).expect("Failed.");
    let type_of = type_of.trim();

    println!("What is the current balance of the account?: ");
    let mut starting_balance = String::new();
    io::stdin()
        .read_line(&mut starting_balance)
        .expect("Failed.");

    let starting_balance: i32 = starting_balance
        .trim()
        .parse()
        .expect("Please type a number!");

    println!("What is the interest rate on this account?: ");
    let mut interest_rate = String::new();
    io::stdin().read_line(&mut interest_rate).expect("Failed.");

    let interest_rate: u32 = interest_rate.trim().parse().expect("Please type a number!");

    liabilities.push(Liability {
        name: name.to_string(),
        type_of: type_of.to_string(),
        starting_balance,
        current_balance: starting_balance,
        interest_rate,
        transactions: Vec::new(),
    });
}

fn execute_transaction(assets: &mut Vec<Asset>) {
    println!("Is your transaction income or an expense?");
    println!("1) Income.");
    println!("2) Expense.");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed!");
    let answer: u32 = answer.trim().parse().expect("Please enter a number!");

    loop {
        match answer {
            1 => {
                let (transaction, income) = log_income();
                assets[0].transactions.push(transaction);
                assets[0].current_balance += income as i32;
                break;
            }
            2 => {
                let (transaction, expense) = log_expense();
                assets[0].transactions.push(transaction);
                assets[0].current_balance -= expense as i32;
                break;
            }
            _ => break,
        }
    }
}

fn log_income() -> (Transaction, u32) {
    println!("Adding new income...");
    println!("Please enter a name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed.");
    let name = name.trim();

    println!("Who paid ya?: ");
    let mut payee = String::new();
    io::stdin().read_line(&mut payee).expect("Failed.");
    let payee = payee.trim();

    println!("How much did they pay ya?: ");
    let mut inflow = String::new();
    io::stdin().read_line(&mut inflow).expect("Failed.");
    let inflow: i32 = inflow.trim().parse().expect("Please type a number!");

    println!("Any thing else you wanna write down?: ");
    let mut notes = String::new();
    io::stdin().read_line(&mut notes).expect("Failed.");
    let notes = notes.trim();

    let income = Transaction::Income {
        name: name.to_string(),
        payee: payee.to_string(),
        inflow: inflow as u32,
        notes: notes.to_string(),
        date: Local::now(),
    };

    return (income, inflow as u32);
}

fn log_expense() -> (Transaction, u32) {
    println!("Adding a new expense...");
    println!("Please enter a name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed.");
    let name = name.trim();

    println!("Who'd ya pay?: ");
    let mut payee = String::new();
    io::stdin().read_line(&mut payee).expect("Failed.");
    let payee = payee.trim();

    println!("How much did ya pay 'em?: ");
    let mut outflow = String::new();
    io::stdin().read_line(&mut outflow).expect("Failed.");
    let outflow: i32 = outflow.trim().parse().expect("Please type a number!");

    println!("Any thing else you wanna write down?: ");
    let mut notes = String::new();
    io::stdin().read_line(&mut notes).expect("Failed.");
    let notes = notes.trim();

    let inflow = outflow * -1;

    let expense = Transaction::Expense {
        name: name.to_string(),
        payee: payee.to_string(),
        inflow,
        outflow,
        notes: notes.to_string(),
        date: Local::now(),
    };

    return (expense, outflow as u32);
}

fn display_transactions(transactions: Vec<Transaction>) {
    for transaction in transactions {
        match transaction {
            Transaction::Income {
                name,
                payee,
                inflow,
                notes,
                date,
            } => {
                println!("Name: {}", name);
                println!("Payee: {}", payee);
                println!("Inflow: ${}", inflow);
                println!("Notes?: {}", notes);
                println!("Date: {}", date);
            }
            Transaction::Expense {
                name,
                payee,
                inflow,
                outflow,
                notes,
                date,
            } => {
                println!("Name: {}", name);
                println!("Payee: {}", payee);
                println!("Inflow: ${}", inflow);
                println!("Outflow: ${}", outflow);
                println!("Notes?: {}", notes);
                println!("Date: {}", date);
            }
        }
        println!();
    }
}

fn display_assets(assets: &Vec<Asset>) {
    for asset in assets {
        println!("Name: {}", asset.name);
        println!("Type: {}", asset.type_of);
        println!("Starting Balance: ${}", asset.starting_balance);
        println!("Current Balance: ${}", asset.current_balance);
        println!();
        println!(
            "Transaction history for {} {}:",
            asset.name, asset.type_of
        );
        display_transactions(asset.transactions.clone());
    }
    println!();
}

fn display_liabilities(liabilities: &Vec<Liability>) {
    for liability in liabilities {
        println!("Name: {}", liability.name);
        println!("Type: {}", liability.type_of);
        println!("Starting Balance: ${}", liability.starting_balance);
        println!("Current Balance: ${}", liability.current_balance);
        println!("Interest Rate: {}%", liability.interest_rate);
        println!();
        println!(
            "Transaction history for {} {}:",
            liability.name, liability.type_of
        );
        display_transactions(liability.transactions.clone());
    }
    println!();
}
