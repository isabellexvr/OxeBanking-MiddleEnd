use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDTO {
    pub descricao: String,
    pub id_usuario: i32,
    pub metodo_pagamento: String,
    pub valor: String,
    pub saldo: String,
    pub destinatario: String,
}

/* {
    "descricao": "pagamento foda",
    "id_usuario": "1",
    "metodo_pagamento": "pix",
    "valor": "50.00",
    "saldo": "100.00",
    "destinatario": "1"
  } */