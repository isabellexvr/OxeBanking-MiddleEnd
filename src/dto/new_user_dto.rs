use serde::{Serialize, Deserialize};

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
    pub full_name: String,
    pub profile_pic: String,
    pub cpf: String,
    pub birthdate: String, // Use a string in ISO 8601 format
    pub marital_status: String,
    pub gross_mensal_income: i32,
    pub email: String,
    pub phone_number: String,
    pub is_admin: bool,
    pub is_blocked: bool,
    pub user_password: String,
    pub address: Address,
}
