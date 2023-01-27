DROP TABLE items;
CREATE TABLE IF NOT EXISTS items
(
    id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title       VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);

INSERT INTO items (title, description)
VALUES ('foo1', 'foobar');
INSERT INTO items (title, description)
VALUES ('foo2', 'foobar');
INSERT INTO items (title, description)
VALUES ('foo3', 'foobar');
INSERT INTO items (title, description)
VALUES ('foo4', 'foobar');
INSERT INTO items (title, description)
VALUES ('foo5', 'foobar');

