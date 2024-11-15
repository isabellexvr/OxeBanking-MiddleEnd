use serde::{Serialize, Deserialize};

/*

Rota: POST /api/credit-card
Parâmetros de Query:

newUserId (Int) - ID do usuário para quem o cartão será criado
closingDay (Int) - Dia de fechamento do cartão

body:
{
  "id": 1,
  "numeroCartao": "1234-5678-9012-3456",
  "cvv": "123",
  "dataValidade": "2025-12-31",
  "limiteDisponivel": 5000.00,
  "status": "ACTIVE",
  "limiteTotal": 5000.00,
  "idUsuario": 1,
  "closingDay": 15
}


Rota: GET /api/credit-card/{id}
Parâmetros de Caminho:

id (Int) - ID do cartão de crédito a ser recuperado



deletar um cartão de crédito por ID
Rota: DELETE /api/credit-card/{id}
Parâmetros de Caminho:

id (Int) - ID do cartão de crédito a ser deletado



Atualizar o limite de crédito de um cartão
Rota: PATCH /api/credit-card/{id}/limit
Parâmetros de Caminho:

id (Int) - ID do cartão de crédito a ser atualizado
Parâmetros de Query:
limiteTotal (Double) - Novo limite total a ser definido

*/

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditCardDTO {
    pub id: Option<i32>,
    pub numeroCartao: String,
    pub cvv: String,
    pub dataValidade: String,
    pub limiteDisponivel: f64,
    pub status: String,
    pub limiteTotal: f64,
    pub idUsuario: i32,
    pub closingDay: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditCardNoUserId {
    pub id: i32,
    pub numeroCartao: String,
    pub cvv: String,
    pub dataValidade: String,
    pub limiteDisponivel: f64,
    pub status: String,
    pub limiteTotal: f64,
    pub closingDay: i32,
}