-- Your SQL goes here
DROP TABLE images;
DROP TABLE posts;
CREATE TABLE posts (
    id INTEGER PRIMARY KEY NOT NULL,
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
