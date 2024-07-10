-- Your SQL goes here
CREATE TABLE accounts (
    user_id VARCHAR(255) PRIMARY KEY,
    account_number VARCHAR(255) NOT NULL,
    ifsc VARCHAR(255) NOT NULL,
    bank_name VARCHAR(255) NOT NULL,
    upi_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);
