use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub zip_code: String,
    pub city: String,
    pub state: String,
    pub uf: String,
    pub street: String,
    pub number: String,
    pub complement: Option<String>,
    pub is_main: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
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
    pub address: Address,
}
