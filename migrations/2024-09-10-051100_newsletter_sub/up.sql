-- Your SQL goes here
CREATE TABLE newsletter_sub (
    email VARCHAR(255) NOT NULL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);