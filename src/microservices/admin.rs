use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::user::{User, Address};
use crate::dto::new_user_dto::UserDTO;
use crate::schema::{users, addresses};
use crate::errors::microservices_errors::ParseError;

pub fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn get_user_by_cpf(cpf: &str) -> Result<Option<User>, ParseError> {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let result = users
        .filter(cpf.eq(cpf))
        .select(User::as_select())
        .first::<User>(&mut connection)
        .optional()
        .map_err(|e| ParseError::Custom(e.to_string()))?;


    Ok(result)
}

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