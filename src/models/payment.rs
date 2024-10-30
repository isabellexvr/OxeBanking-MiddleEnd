use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: i32,
    pub descricao: String,
    pub id_usuario: i32,
    pub metodo_pagamento: String,
    pub valor: f64,
    pub saldo: f64,
    pub destinatario: i32,
    pub inserted_at: String,
    pub updated_at: String,
}