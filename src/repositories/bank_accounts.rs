use crate::schema::{account, br_account, global_account};
use crate::repositories::connection::establish_connection;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use serde::{Serialize, Deserialize};
use crate::errors::microservices_errors::ParseError;

#[derive(Queryable, QueryableByName, Selectable, Serialize, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = account)]
pub struct Account {
    pub id: Option<i32>,
    pub user_id: i32,
    pub balance: i32,
    pub gross_mensal_income: i32,
}

#[derive(Queryable, QueryableByName, Selectable, Serialize, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = br_account)]
struct BrAccount {
    id: Option<i32>,
    account_id: i32,
    account_number: String,
    agency: String,
    account_type: String,
    bank_name: String,
    bank_code: String,
}

#[derive(Queryable, QueryableByName, Selectable, Serialize, Insertable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = global_account)]
struct GlobalAccount {
    id: Option<i32>,
    account_id: i32,
    account_number: String,
    ach_routing_number: String,
    wire_transfer_routing_number: String,
    bank_name: String,
    bank_code: String,
    bank_address: String,
}


pub async fn create_new_account(user_id: i32, gross_mensal_income: i64) -> Result<String, ParseError> {
    let mut connection = establish_connection();


    // Insert into `account`
    diesel::insert_into(account::table)
        .values(&Account {
            id: None,
            user_id,
            balance: 0,
            gross_mensal_income: gross_mensal_income as i32,
        })
        .execute(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?;

    
    // Retrieve the last inserted account's ID
    let new_account_id: i32 = account::table
        .select(account::id)
        .order(account::id.desc())
        .first::<Option<i32>>(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?
        .unwrap();

    // Insert into `br_account` using the retrieved `account_id`
    diesel::insert_into(br_account::table)
        .values(&BrAccount {
            id: None,
            account_id: new_account_id, // Use the created account's `id`
            account_number: "123456".to_string(),
            agency: "1234".to_string(),
            account_type: "Conta Corrente".to_string(),
            bank_name: "Banco do Brasil".to_string(),
            bank_code: "001".to_string(),
        })
        .execute(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?;


    diesel::insert_into(global_account::table)
        .values(&GlobalAccount {
            id: None,
            account_id: new_account_id, // Use the created account's `id`
            account_number: "123456".to_string(),
            ach_routing_number: "123456".to_string(),
            wire_transfer_routing_number: "123456".to_string(),
            bank_name: "Banco do Brasil".to_string(),
            bank_code: "001".to_string(),
            bank_address: "Rua do Banco do Brasil".to_string(),
        })
        .execute(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?;

    Ok("User created successfully!".to_string())
}

pub async fn get_account_by_user_id(user_id: i32) -> Result<Account, ParseError> {
    let mut connection = establish_connection();

    let result = account::table
        .filter(account::user_id.eq(user_id))
        .first::<Account>(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?;

    Ok(result)
}

pub async fn get_user_balance(user_id: i32) -> Result<i32, ParseError> {
    let mut connection = establish_connection();

    let result = account::table
        .filter(account::user_id.eq(user_id))
        .select(account::balance)
        .first::<i32>(&mut connection)
        .unwrap_or(0);

    Ok(result)
}

