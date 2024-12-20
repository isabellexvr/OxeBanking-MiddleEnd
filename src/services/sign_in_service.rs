use actix_web::error::Error;
use crate::models::user::User; // Ensure User is imported
use crate::microservices::admin::{get_user_by_cpf}; // Ensure get_user_by_id is imported
use bcrypt::{verify as bcrypt_verify}; // Import the bcrypt verification function
use jsonwebtoken::{encode, Header}; // Import necessary items for JWT
use crate::services::auth_service::create_jwt_token; // Import your token creation function
use actix_web::web;
use crate::dto::signin_dto::SignInDTO;


pub async fn sign_in_service(credentials: web::Json<SignInDTO>) -> Result<(String), Error> {
    // Call get_user_by_id asynchronously and await the result
    
    match get_user_by_cpf(&credentials.cpf).await {
        Ok(Some(user)) => {
            // User was found, now verify the password
            if bcrypt_verify(&credentials.user_password, &user.user_password).map_err(|_| {
                actix_web::error::ErrorInternalServerError("Incorrect Password")
            })? {
                // Passwords match, create a JWT token
                match user.id {
                    Some(id) => match create_jwt_token(&user.full_name, id, &user.profile_pic) {
                        Ok(token) => {
                            // Return the user and token
                            Ok((token))
                        }
                        Err(_) => {
                            // Handle error in token creation
                            Err(actix_web::error::ErrorInternalServerError("Error creating token"))
                        }
                    },
                    None => Err(actix_web::error::ErrorInternalServerError("User ID is missing")),
                }
                
            } else {
                // Passwords do not match
                Err(actix_web::error::ErrorUnauthorized("Invalid password")) // Return unauthorized error
            }
        }
        Ok(None) => {
            // User was not found
            Err(actix_web::error::ErrorNotFound("User not found")) // Use ErrorNotFound directly
        }
        Err(err) => {
            // Handle the error (e.g., logging)
            eprintln!("Error while fetching user: {:?}", err);
            Err(actix_web::error::ErrorInternalServerError("Internal server error")) // Use ErrorInternalServerError directly
        }
    }
}
