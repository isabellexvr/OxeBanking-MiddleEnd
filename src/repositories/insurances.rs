use diesel::prelude::*;
use crate::schema::{insurances, insurance_types, insurance_descriptions};
use diesel::dsl::sql;
use crate::models::insurances::InsuranceResponse;
use serde::{Serialize, Deserialize};
use diesel::sql_types::Text;
use crate::repositories::connection::establish_connection;

pub fn get_insurance_details(insurance_id: i32) -> Result<InsuranceResponse, diesel::result::Error> {
    let mut conn = establish_connection();
    let insurance_data = insurances::table
        .inner_join(insurance_types::table.on(insurance_types::id.eq(insurances::type_id.nullable())))
        .inner_join(insurance_descriptions::table.on(insurance_descriptions::insurance_id.nullable().eq(insurances::id)))
        .filter(insurances::id.eq(insurance_id))
        .select((
            insurances::title,
            insurance_types::icon,
            insurances::price,
            insurances::contracted.nullable(), // Ajuste para Nullable
            insurance_types::type_,
            sql::<Text>("GROUP_CONCAT(insurance_descriptions.description, ',') AS description")
        ))
        .first::<(String, String, f64, Option<bool>, String, String)>(&mut conn)?;

    // Separe as descrições e converta em um vetor
    let (title, icon, price, contracted, insurance_type, description_str) = insurance_data;
    let description = description_str.split(',').map(String::from).collect();

    Ok(InsuranceResponse {
        title,
        icon,
        price,
        contracted: contracted.unwrap_or(false),
        insurance_type,
        description,
    })
}

#[derive(Queryable, serde::Serialize)]
pub struct Insurance {
    pub id: Option<i32>,
    pub title: String,
    pub type_: String,
    pub price: f64,
    pub contracted: Option<bool>,
    pub description: String, // Alteração aqui para Vec<String>
    pub icon: String,
}

#[derive(Queryable, serde::Serialize)]
pub struct SaidaInsurances {
    pub id: Option<i32>,
    pub title: String,
    pub type_: String,
    pub price: f64,
    pub contracted: Option<bool>,
    pub description: Vec<String>, // Alteração aqui para Vec<String>
    pub icon: String,
}

#[derive(Queryable, QueryableByName, Selectable, Serialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = insurance_descriptions)]
struct InsuranceDescription {
    pub id: Option<i32>,
    insurance_id: i32,
    description: String,
}

#[derive(Serialize, Queryable, Debug)]
struct InsuranceWithDescriptions {
    id: i32,
    title: String,
    price: f64,
    contracted: Option<bool>,
    type_: String,
    icon: String,
    descriptions: Vec<String>,
}

pub fn get_all_insurances_repository() -> Result<Vec<SaidaInsurances>, diesel::result::Error> {
    let mut conn = establish_connection();

    let results = insurances::table
        .inner_join(insurance_types::table.on(insurances::type_id.nullable().eq(insurance_types::id)))
        .inner_join(insurance_descriptions::table.on(insurances::id.nullable().eq(insurance_descriptions::insurance_id.nullable())))
        .select((
            insurances::id,
            insurances::title,
            insurance_types::type_,
            insurances::price,
            insurances::contracted,
            insurance_descriptions::description,
            insurance_types::icon,
        ))
        .load::<Insurance>(&mut conn)?;
    let descriptions = insurance_descriptions::table
            .select(InsuranceDescription::as_select())
            .load::<InsuranceDescription>(&mut conn)?;


    let mut insurance_map: std::collections::HashMap<i32, InsuranceWithDescriptions> = std::collections::HashMap::new();

    for insurance in results {
        insurance_map.insert(
            insurance.id.expect("Insurance ID should be present"),
            InsuranceWithDescriptions {
                id: insurance.id.expect("Insurance ID should be present"),
                title: insurance.title,
                price: insurance.price,
                contracted: insurance.contracted,
                type_: insurance.type_,
                icon: insurance.icon,
                descriptions: Vec::new(),
            },
        );
    }

    for description in descriptions {
        if let Some(insurance) = insurance_map.get_mut(&description.insurance_id) {
            insurance.descriptions.push(description.description);
        }
    }

    Ok(insurance_map.into_iter().map(|(_, v)| SaidaInsurances {
        id: Some(v.id),
        title: v.title,
        type_: v.type_,
        price: v.price,
        contracted: v.contracted,
        description: v.descriptions,
        icon: v.icon,
    }).collect())
}