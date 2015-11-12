-- name: drop-table-person
DROP TABLE IF EXISTS "person" CASCADE;

-- name: create-table-person
CREATE TABLE "person" (id SERIAL PRIMARY KEY, name  VARCHAR NOT NULL, data BYTEA);

-- name: insert-person
INSERT INTO "person" (name, data) VALUES ($1, $2);

-- name: select-all
SELECT id, name, data FROM person;
