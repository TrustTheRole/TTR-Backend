-- Your SQL goes here
CREATE TABLE colleges (
    id VARCHAR(255) PRIMARY KEY,
    college_name VARCHAR(255) UNIQUE NOT NULL,
    college_location VARCHAR(255) NOT NULL,
    students_registered INT NOT NULL
);