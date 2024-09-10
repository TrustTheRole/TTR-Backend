-- Your SQL goes here
CREATE TABLE likes (
  insight_id VARCHAR(255) NOT NULL PRIMARY KEY,
  like_count INT DEFAULT 0 NOT NULL,
  view_count INT DEFAULT 0 NOT NULL,
  FOREIGN KEY (insight_id) REFERENCES insights(insight_id)
);