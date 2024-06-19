-- Your SQL goes here
CREATE TABLE tokens (
    id SERIAL PRIMARY KEY,
    account_name VARCHAR(255) NOT NULL,
    issuer VARCHAR(50) NOT NULL,
    secret VARCHAR(64) NOT NULL
)
