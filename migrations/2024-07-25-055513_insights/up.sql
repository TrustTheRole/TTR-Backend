-- Your SQL goes here
CREATE TABLE insights (
    insight_id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    insight_title VARCHAR(255) NOT NULL,
    insight_company VARCHAR(255) NOT NULL,
    insight_role VARCHAR(255) NOT NULL,
    insight_tags TEXT[] NOT NULL,
    insight_description TEXT NOT NULL,
    insight_picture_urls TEXT[] NOT NULL,
    insight_focus_points TEXT[] NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);