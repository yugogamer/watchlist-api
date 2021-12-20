-- Your SQL goes here

CREATE TABLE account (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(75) NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE session (
    id SERIAL,
    id_account INT4 NOT NULL,
    PRIMARY KEY(id, id_account),
    CONSTRAINT fk_account_session
        FOREIGN KEY(id_account)
            REFERENCES account(id)
);