CREATE TABLE post_view (
    id INTEGER PRIMARY KEY,
    post_id INTEGER NOT NULL,
    visitor_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (post_id) REFERENCES post(id),
    FOREIGN KEY (visitor_id) REFERENCES visitor(id)
);
