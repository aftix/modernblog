-- Your SQL goes here
DROP TABLE images;
ALTER TABLE posts ADD COLUMN header VARCHAR;
CREATE TABLE images (
    id INTEGER PRIMARY KEY,
    name VARCHAR NOT NULL,
    postid INTEGER NOT NULL,
    FOREIGN KEY (postid) REFERENCES posts(id)
);
