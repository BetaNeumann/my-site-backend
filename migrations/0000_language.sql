CREATE TABLE language (
    id INTEGER PRIMARY KEY,
    description TEXT NOT NULL,
    acronym TEXT NOT NULL
);

INSERT INTO language (description, acronym) VALUES ("Português Brasileiro", "pt-br");
INSERT INTO language (description, acronym) VALUES ("English", "en");
