-- Your SQL goes here
CREATE TABLE series(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    rating INT NOT NULL
)