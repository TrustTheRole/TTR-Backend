-- Your SQL goes here
CREATE TABLE users (
    user_id VARCHAR(255) PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    alternate_email VARCHAR(255),
    phone VARCHAR(50) NOT NULL,
    college VARCHAR(255) NOT NULL,
    graduation_year INT NOT NULL,
    linkedin VARCHAR(255),
    github VARCHAR(255)
);
