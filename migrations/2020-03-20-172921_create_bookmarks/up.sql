CREATE TABLE bookmarks
(
    id          INTEGER PRIMARY KEY,
    key         VARCHAR(10)  NOT NULL,
    path        VARCHAR(255) NOT NULL,
    description TEXT
);
