use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    zip_code: String,
    city: String,
    state: String,
    uf: String,
    street: String,
    number: String,
    complement: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    full_name: String,
    profile_pic: String,        // Could be validated as a URL in actual code
    cpf: String,
    birthdate: NaiveDate,       // Requires 'chrono' for handling dates
    marital_status: String,     // Will come from the select in the frontend
    gross_mensal_income: f64,   // Double type in Rust
    email: String,
    phone_number: String,
    is_admin: bool,                 
    is_blocked: bool,
    user_password: String,
    created_at: NaiveDate,
    updated_at: NaiveDate,
    address: Address,           // Embedded object
}
