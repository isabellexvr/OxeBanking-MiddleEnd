use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentBD {
    pub id: i32,
    pub descricao: String,
    pub id_usuario: i32,
    pub metodo_pagamento: String,
    pub valor: String,
    pub saldo: String,
    pub fatura_pendente: Option<String>,
    pub destinatario: i32,
    pub inserted_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: i32,
    pub descricao: String,
    pub id_usuario: i32,
    pub metodo_pagamento: String,
    pub valor: i32,
    pub saldo: i32,
    pub fatura_pendente: Option<i32>,
    pub destinatario: i32, // poderia ser um option string ou int (destinatario do oxebanking ou externo)
    pub inserted_at: String,
    pub updated_at: String,
}