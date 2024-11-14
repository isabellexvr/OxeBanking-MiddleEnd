use diesel::sql_types::{Text, Double, Nullable, Bool};
use diesel::QueryableByName;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
pub struct InsuranceResponse {
    #[diesel(sql_type = Text)]
    pub title: String,
    
    #[diesel(sql_type = Text)]
    pub icon: String,
    
    #[diesel(sql_type = Text)]
    pub description: Vec<String>,
    
    #[diesel(sql_type = Double)]
    pub price: f64,
    
    #[diesel(sql_type = Nullable<Bool>)]
    pub contracted: bool,
    
    #[diesel(sql_type = Text)]
    pub insurance_type: String,
}

