# Como rodar?
- cargo clean
- cargo build
- cargo run

# Como rodar para desenvolvimento? (reiniciar o server a cada mudança)
- cargo install cargo-watch
- cargo watch -x 'run'





o que falta de pagamentos?
- pagamento de boleto (codigo de barras)
- rota de pegar todos os pagamentos de um user_id especifico
- mudar a logica do campo "destinatario"

{
  "descricao": "Pagamento de conta",
  "id_usuario": 12345,
  "metodo_pagamento": "boleto",
  "valor": "100.00",
  "saldo": "200.00",
  "destinatario": "codigo de barras aqui?"
}


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