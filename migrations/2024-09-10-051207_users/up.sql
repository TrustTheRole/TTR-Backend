-- Your SQL goes here
CREATE TABLE users (
    user_id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(50),
    email VARCHAR(255) NOT NULL UNIQUE,
    alternate_email VARCHAR(255),
    phone VARCHAR(50),
    college VARCHAR(255),
    graduation_year INT,
    linkedin VARCHAR(255),
    github VARCHAR(255),
    gender VARCHAR(50) NOT NULL,
    liked_insights TEXT[] NOT NULL,
    viewed_insights TEXT[] NOT NULL,
    FOREIGN KEY (college) REFERENCES colleges(college_name)
);