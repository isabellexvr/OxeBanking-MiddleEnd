// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (id) {
        id -> Nullable<Integer>,
        zip_code -> Text,
        city -> Text,
        state -> Text,
        uf -> Text,
        street -> Text,
        number -> Text,
        complement -> Nullable<Text>,
        is_main -> Bool,
    }
}

diesel::table! {
    insurance_descriptions (id) {
        id -> Nullable<Integer>,
        insurance_id -> Integer,
        description -> Text,
    }
}

diesel::table! {
    insurance_types (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Text,
        icon -> Text,
    }
}

diesel::table! {
    insurances (id) {
        id -> Nullable<Integer>,
        title -> Text,
        price -> Double,
        contracted -> Nullable<Bool>,
        type_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        full_name -> Text,
        profile_pic -> Text,
        cpf -> Text,
        birthdate -> Text,
        marital_status -> Text,
        gross_mensal_income -> Integer,
        email -> Text,
        phone_number -> Text,
        is_admin -> Bool,
        is_blocked -> Bool,
        user_password -> Text,
        created_at -> Text,
        updated_at -> Text,
        address_id -> Integer,
    }
}

diesel::joinable!(insurance_descriptions -> insurances (insurance_id));
diesel::joinable!(insurances -> insurance_types (type_id));
diesel::joinable!(users -> addresses (address_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    insurance_descriptions,
    insurance_types,
    insurances,
    users,
);
