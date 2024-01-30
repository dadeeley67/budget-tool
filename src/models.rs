use crate::schema::{accounts, transactions};

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub type_of: &'a str,
    pub starting_balance: i32,
    pub current_balance: i32,
}

#[derive(Debug, Queryable, AsChangeset, Selectable)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub type_of: String,
    pub starting_balance: i32,
    pub current_balance: i32,
}

#[derive(Insertable)]
#[diesel(table_name = transactions)]
pub struct NewTransaction<'a> {
    pub name: &'a str,
    pub payee: &'a str,
    pub inflow: i32,
    pub outflow: i32,
    pub notes: &'a str,
}

#[derive(Debug, Queryable, AsChangeset, Selectable)]
pub struct Transaction {
    pub id: i32,
    pub name: String,
    pub payee: String,
    pub inflow: i32,
    pub outflow: i32,
    pub notes: String,
}
