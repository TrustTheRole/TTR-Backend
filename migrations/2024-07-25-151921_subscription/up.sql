-- Your SQL goes here
CREATE TABLE subscription (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);