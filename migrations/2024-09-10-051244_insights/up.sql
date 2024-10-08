-- Your SQL goes here
CREATE TABLE insights (
    insight_id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    user_name VARCHAR(255) NOT NULL,
    insight_title VARCHAR(255) NOT NULL,
    insight_company VARCHAR(255) NOT NULL,
    insight_role VARCHAR(255) NOT NULL,
    insight_tags TEXT[] NOT NULL,
    insight_description TEXT NOT NULL,
    insight_picture_urls TEXT[] NOT NULL,
    insight_focus_points TEXT[] NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (insight_company) REFERENCES companies(company_name)
);