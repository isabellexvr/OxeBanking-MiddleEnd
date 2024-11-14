CREATE TABLE account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    balance INTEGER NOT NULL,
    gross_mensal_income INTEGER NOT NULL
);

CREATE TABLE account_pix_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    pix_key TEXT NOT NULL,
    key_type TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account(id)
);

CREATE TABLE br_account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    account_number TEXT NOT NULL,
    agency TEXT NOT NULL,
    account_type TEXT NOT NULL,
    bank_name TEXT NOT NULL,
    bank_code TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account(id)
);

CREATE TABLE global_account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    account_number TEXT NOT NULL,
    ach_routing_number TEXT NOT NULL,
    wire_transfer_routing_number TEXT NOT NULL,
    bank_name TEXT NOT NULL,
    bank_code TEXT NOT NULL,
    bank_address TEXT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES account(id)
);