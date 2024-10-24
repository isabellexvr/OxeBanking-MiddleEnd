use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub profile_pic: String,
    pub cpf: String,
    pub birthdate: String, // Use a string in ISO 8601 format
    pub marital_status: String,
    pub gross_mensal_income: f64,
    pub email: String,
    pub phone_number: String,
    pub is_admin: bool,
    pub is_blocked: bool,
    pub user_password: String,
    pub created_at: String,
    pub updated_at: String,
    pub address_id: i32, // Assuming you have the address ID available
}
