use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInDTO {
    pub cpf : String,
    pub user_password : String,
}
