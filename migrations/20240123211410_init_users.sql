-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    "id" VARCHAR(64) NOT NULL PRIMARY KEY,
    "username" VARCHAR(64) NOT NULL UNIQUE,
    "password" VARCHAR(64)
);