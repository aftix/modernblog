-- Your SQL goes here
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    draft BOOLEAN NOT NULL,
    time DATETIME NOT NULL
);
