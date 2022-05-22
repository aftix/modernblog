-- This file should undo anything in `up.sql`
DROP TABLE images;
CREATE TABLE images (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    postid INTEGER NOT NULL,
    FOREIGN KEY (postid) REFERENCES posts(id)
);