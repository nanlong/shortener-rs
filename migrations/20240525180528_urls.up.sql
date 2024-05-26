-- Add up migration script here
CREATE TABLE IF NOT EXISTS urls (
    id CHAR(6) PRIMARY KEY,
    url TEXT NOT NULL UNIQUE
);
