//Aqui definimos o modelo de usuário, que deve possuir username, email e password
//Como os dados são representados em nossa modelagem
use serde::{Serialize, Deserialize};

// Estrutura de usuário para login e autenticação
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String, // Senha criptografada
}

// Estrutura de claims para JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
