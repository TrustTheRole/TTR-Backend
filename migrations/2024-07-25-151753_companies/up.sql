-- Your SQL goes here
CREATE TABLE companies (
    id SERIAL PRIMARY KEY,
    company_name VARCHAR(255) UNIQUE NOT NULL
);