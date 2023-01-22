CREATE TABLE key_pairs (
  "id" SERIAL PRIMARY KEY,
  "key" VARCHAR NOT NULL,
  "token" VARCHAR NOT NULL,
  "name" VARCHAR NOT NULL,
  "permission" INTEGER NOT NULL
)