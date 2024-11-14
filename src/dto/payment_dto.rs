use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDTO {
    pub descricao: String,
    pub id_usuario: String,
    pub metodo_pagamento: String,
    pub valor: String,
    pub saldo: String,
    pub destinatario: String,
}
