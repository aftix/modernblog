-- Your SQL goes here
DROP TABLE images;
CREATE TABLE images (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    postid INTEGER NOT NULL,
    FOREIGN KEY (postid) REFERENCES posts(id)
);