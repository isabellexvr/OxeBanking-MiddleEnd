-- migrations/{timestamp}_create_users_and_addresses_tables.sql

-- Create the addresses table
CREATE TABLE addresses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    zip_code TEXT NOT NULL,
    city TEXT NOT NULL,
    state TEXT NOT NULL,
    uf TEXT NOT NULL,
    street TEXT NOT NULL,
    number TEXT NOT NULL,
    complement TEXT,
    is_main BOOLEAN NOT NULL
);

-- Create the users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    full_name TEXT NOT NULL,
    profile_pic TEXT NOT NULL,
    cpf TEXT NOT NULL,
    birthdate TEXT NOT NULL, -- ISO 8601 format
    marital_status TEXT NOT NULL,
    gross_mensal_income INTEGER NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL,
    is_blocked BOOLEAN NOT NULL,
    user_password TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    address_id INTEGER NOT NULL,
    FOREIGN KEY (address_id) REFERENCES addresses (id)
);