-- This file should undo anything in `up.sql`
DROP TABLE images;
ALTER TABLE posts DROP COLUMN header;
CREATE TABLE images (
    id INTEGER PRIMARY KEY,
    location VARCHAR NOT NULL,
    refs INTEGER NOT NULL,
    hash VARCHAR NOT NULL
);
