use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountBD{
    pub id: i32,
    pub user_id: i32,
    pub balance: i32,
    pub gross_mensal_income: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BRAccountBD {
    pub id: i32,
    pub account_id: i32,
    pub account_number: String,
    pub agency: String,
    pub account_type: String,
    pub bank_name: String,
    pub bank_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalAccountBD {
    pub id: i32,
    pub account_id: i32,
    pub account_number: String,
    pub ach_routing_number: String,
    pub wire_transfer_routing_number: String,
    pub bank_name: String,
    pub bank_code: String,
    pub bank_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountPixKeysBD {
    pub id: i32,
    pub account_id: i32,
    pub pix_key: String,
    pub key_type: String,
}