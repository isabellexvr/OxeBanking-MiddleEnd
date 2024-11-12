use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{addresses, users};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[table_name = "addresses"]
pub struct Address {
    pub id: Option<i32>,
    pub zip_code: String,
    pub city: String,
    pub state: String,
    pub uf: String,
    pub street: String,
    pub number: String,
    pub complement: Option<String>,
    pub is_main: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, Associations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[belongs_to(Address)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub full_name: String,
    pub profile_pic: String,
    pub cpf: String,
    pub birthdate: String,
    pub marital_status: String,
    pub gross_mensal_income: i32,
    pub email: String,
    pub phone_number: String,
    pub is_admin: bool,
    pub is_blocked: bool,
    pub user_password: String,
    pub created_at: String,
    pub updated_at: String,
    pub address_id: i32,
}
