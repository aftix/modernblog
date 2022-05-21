-- This file should undo anything in `up.sql`
DROP table images;
DROP table posts;
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    draft BOOLEAN NOT NULL,
    time DATETIME NOT NULL,
    header VARCHAR
);
CREATE TABLE images (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    postid INTEGER NOT NULL,
    FOREIGN KEY (postid) REFERENCES posts(id)
);
