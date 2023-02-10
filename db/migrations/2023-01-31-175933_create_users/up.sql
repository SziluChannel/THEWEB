-- Creating table users
CREATE TABLE users (
    ID uuid DEFAULT gen_random_uuid () NOT NULL UNIQUE, --SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    admin BOOLEAN DEFAULT FALSE NOT NULL,
    confirmation_token VARCHAR(50) NOT NULL UNIQUE,
    confirmed BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (ID)
);


-- creating admin user for testing purposes
INSERT INTO users
VALUES (
    gen_random_uuid (),
    'Szilumester',
    'channelszilu@gmail.com',
    '$argon2id$v=19$m=4096,t=3,p=1$RyFKNGtmNGczbGY0MzRma2pLRiUhWkpnSyFSSzV+$XnXeLQVcm59/IPt+tCGmukzUwyi2KQL/dH2DxfMVZo0',
    true,
    'A Mesternek nem kő ilyen h TokEN',
    true
);

-- creating non-admin user for testing purposes
INSERT INTO users
VALUES (
    gen_random_uuid (),
    'NyalTamás',
    'nyaltamas@gmail.com',
    '$argon2id$v=19$m=4096,t=3,p=1$RyFKNGtmNGczbGY0MzRma2pLRiUhWkpnSyFSSzV+$XnXeLQVcm59/IPt+tCGmukzUwyi2KQL/dH2DxfMVZo0',
    false,
    'A Slavenek nem kő ilyen h TokEN',
    true
);
