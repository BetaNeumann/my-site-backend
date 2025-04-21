CREATE TABLE post_content (
    id INTEGER PRIMARY KEY,
    post_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT false,
    reading_time INTEGER DEFAULT NULL,

    FOREIGN KEY (post_id) REFERENCES post(id),
    FOREIGN KEY (language_id) REFERENCES language(id)
);
