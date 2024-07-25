-- Your SQL goes here
CREATE TABLE colleges (
    id SERIAL PRIMARY KEY,
    college_name VARCHAR(255) UNIQUE NOT NULL,
    college_location VARCHAR(255) NOT NULL,
    college_state VARCHAR(255) NOT NULL
);