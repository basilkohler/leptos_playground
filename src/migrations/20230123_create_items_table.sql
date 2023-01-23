CREATE TABLE IF NOT EXISTS items
(
    id          INTEGER NOT NULL PRIMARY KEY,
    title       VARCHAR,
    description VARCHAR
);

INSERT INTO items (id, title, description)
VALUES (1, 'foo1', 'foobar');
INSERT INTO items (id, title, description)
VALUES (2, 'foo2', 'foobar');
INSERT INTO items (id, title, description)
VALUES (3, 'foo3', 'foobar');
INSERT INTO items (id, title, description)
VALUES (4, 'foo4', 'foobar');
INSERT INTO items (id, title, description)
VALUES (5, 'foo5', 'foobar');