-- Your SQL goes here
CREATE TABLE companies (
    id VARCHAR(255) PRIMARY KEY,
    company_name VARCHAR(255) UNIQUE NOT NULL
);