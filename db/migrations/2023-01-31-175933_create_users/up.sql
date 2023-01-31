-- Your SQL goes here
CREATE TABLE users (
    ID uuid DEFAULT gen_random_uuid () NOT NULL, --SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password VARCHAR NOT NULL,
    PRIMARY KEY (ID)
);
