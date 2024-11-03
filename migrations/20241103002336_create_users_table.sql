-- Add migration script here
-- migrations/{timestamp}_create_users_table.sql
CREATE TABLE test (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);