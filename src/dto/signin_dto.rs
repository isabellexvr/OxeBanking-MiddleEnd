use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInDTO {
    pub cpf : String,
    pub user_password : String,
}
