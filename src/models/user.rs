use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub full_name: String,
    pub profile_pic: String,
    pub cpf: String,
    pub birthdate: String, // Use a string in ISO 8601 format
    pub marital_status: String,
    pub gross_mensal_income: i64,
    pub email: String,
    pub phone_number: String,
    pub is_admin: bool,
    pub is_blocked: bool,
    pub user_password: String,
    pub created_at: String,
    pub updated_at: String,
    pub address: Address, // Assuming you have the address ID available
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserBD {
    pub id: i64,
    pub full_name: String,
    pub profile_pic: String,
    pub cpf: String,
    pub birthdate: String, // Use a string in ISO 8601 format
    pub marital_status: String,
    pub gross_mensal_income: i64,
    pub email: String,
    pub phone_number: String,
    pub is_admin: bool,
    pub is_blocked: bool,
    pub user_password: String,
    pub created_at: String,
    pub updated_at: String,
    pub address_id: i64, // Assuming you have the address ID available
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub id: i64,
    pub zip_code: String,
    pub city: String,
    pub state: String,
    pub uf: String,
    pub street: String,
    pub number: String,
    pub complement: Option<String>,
    pub is_main: bool,
}