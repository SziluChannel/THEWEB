-- Your SQL goes here
CREATE TABLE users (
    ID uuid DEFAULT gen_random_uuid () NOT NULL UNIQUE, --SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    PRIMARY KEY (ID)
);
