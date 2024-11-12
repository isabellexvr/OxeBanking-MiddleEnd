use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SignInDTO {
    pub cpf : String,
    pub user_password : String,
}
