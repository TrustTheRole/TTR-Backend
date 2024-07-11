-- Your SQL goes here
CREATE TABLE users (
user_id VARCHAR(255) PRIMARY KEY,
password VARCHAR(255) NOT NULL,
name VARCHAR(255) NOT NULL,
role VARCHAR(50),
email VARCHAR(255) NOT NULL UNIQUE,
alternate_email VARCHAR(255),
phone VARCHAR(50),
college VARCHAR(255),
graduation_year INT,
linkedin VARCHAR(255),
github VARCHAR(255)
);
