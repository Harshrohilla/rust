CREATE TABLE rust_user (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL
);
