-- Your SQL goes here
CREATE TABLE IF NOT EXISTS auth (
    user_pubkey VARCHAR(255) PRIMARY KEY,
    user_address VARCHAR(255) UNIQUE NOT NULL,
    email TEXT UNIQUE,
    twitter TEXT UNIQUE
);
