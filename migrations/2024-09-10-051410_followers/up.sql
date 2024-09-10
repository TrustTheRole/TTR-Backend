-- Your SQL goes here
CREATE TABLE followers (
    user_id VARCHAR(255),
    follower_id VARCHAR(255),
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (follower_id) REFERENCES users(user_id),
    PRIMARY KEY (user_id, follower_id)
);
