-- Your SQL goes here
CREATE TABLE IF NOT EXISTS orders (
    uuid VARCHAR(255) PRIMARY KEY,
    signer TEXT NOT NULL,
    collection TEXT NOT NULL,
    price TEXT NOT NULL,
    token_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    end_time TEXT NOT NULL,
    is_order_ask BOOLEAN NOT NULL,
    signed_msg TEXT NOT NULL,
    makerorder JSON NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
);
