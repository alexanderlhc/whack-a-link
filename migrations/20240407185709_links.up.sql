CREATE TABLE links (
    shortcode TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);