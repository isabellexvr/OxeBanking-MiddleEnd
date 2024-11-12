use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub auth_token: String,
}