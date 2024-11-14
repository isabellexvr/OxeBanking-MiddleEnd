use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::user::{User, Address};
use crate::dto::new_user_dto::UserDTO;
use crate::schema::{users, addresses};
use crate::errors::microservices_errors::ParseError;
use crate::dto::jwt::Claims;
use actix_web::{HttpRequest, Result, HttpMessage};
use serde::{Serialize, Deserialize};
use crate::repositories::bank_accounts::get_user_balance;

pub fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn get_user_by_cpf(cpf_input: &str) -> Result<Option<User>, ParseError> {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let result = users
        .filter(cpf.eq(cpf_input))
        .select(User::as_select())
        .first::<User>(&mut connection)
        .optional()
        .map_err(|e| ParseError::Custom(e.to_string()))?;


    Ok(result)
}

pub async fn get_username_by_id(id: i32) -> Result<String, ParseError> {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let result = users
        .filter(id.eq(id))
        .select(full_name)
        .first::<String>(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?;

    Ok(result)
}

//authenticated route
pub async fn get_user_by_id(id: i32) -> Result<Option<User>, ParseError> {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let result = users
        .filter(id.eq(id))
        .select(User::as_select())
        .first::<User>(&mut connection)
        .optional()
        .map_err(|e| ParseError::Custom(e.to_string()))?;

    Ok(result)
}

pub async fn insert_user(user_info: UserDTO) -> Result<User, ParseError> {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();

    let created_address_id = diesel::insert_into(addresses::table)
        .values(&Address {
            id: None,
            is_main: true,
            street: user_info.address.street.clone(),
            uf: user_info.address.uf.clone(),
            number: user_info.address.number.clone(),
            complement: user_info.address.complement.clone(),
            city: user_info.address.city.clone(),
            state: user_info.address.state.clone(),
            zip_code: user_info.address.zip_code.clone(),
        })
        .execute(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?;

    diesel::insert_into(users)
        .values(&User {
            id: None,
            full_name: user_info.full_name.clone(),
            profile_pic: user_info.profile_pic.clone(),
            cpf: user_info.cpf.clone(),
            birthdate: user_info.birthdate.clone(),
            marital_status: user_info.marital_status.clone(),
            gross_mensal_income: user_info.gross_mensal_income as i32,
            email: user_info.email.clone(),
            phone_number: user_info.phone_number.clone(),
            is_admin: user_info.is_admin,
            is_blocked: user_info.is_blocked,
            user_password: user_info.user_password.clone(),
            address_id: created_address_id as i32,
            created_at: "".to_string(),
            updated_at: "".to_string(),
        })
        .execute(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?;

    let inserted_user = users
        .order(id.desc())
        .first::<User>(&mut connection)
        .map_err(|e| ParseError::Custom(e.to_string()))?;

    Ok(inserted_user)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfos {
    pub id: Option<i32>,
    pub full_name: String,
    pub profile_pic: Option<String>,
    pub cpf: String,
    pub birthdate: String,
    pub marital_status: String,
    pub gross_mensal_income: i32,
    pub email: String,
    pub phone_number: String,
    pub is_admin: bool,
    pub is_blocked: bool,
    pub user_password: String,
    pub address_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub balance: i32, 
}

pub async fn get_authenticated_user(req: HttpRequest) -> Result<UserInfos, ParseError> {

    if let Some(claims) = req.extensions().get::<Claims>() {
        
        let user_id = claims.user_id;
        let user_balance = get_user_balance(user_id).await?;

        let mut connection = establish_connection();
        let user = users::table
            .filter(users::id.eq(user_id))
            .first::<User>(&mut connection)
            .optional()
            .map_err(|e| ParseError::Custom(e.to_string()))?;

        match user {
            Some(user) => {
                let user_infos = UserInfos {
                    id: user.id,
                    full_name: user.full_name,
                    profile_pic: Some(user.profile_pic),
                    cpf: user.cpf,
                    birthdate: user.birthdate,
                    marital_status: user.marital_status,
                    gross_mensal_income: user.gross_mensal_income,
                    email: user.email,
                    phone_number: user.phone_number,
                    is_admin: user.is_admin,
                    is_blocked: user.is_blocked,
                    user_password: user.user_password,
                    address_id: user.address_id,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    balance: user_balance,
                };

                Ok(user_infos)
            },
            None => Err(ParseError::Custom("User not found".to_string())),
        }
    } else {
        Err(ParseError::Custom("Missing or invalid token".to_string()))
    }
}