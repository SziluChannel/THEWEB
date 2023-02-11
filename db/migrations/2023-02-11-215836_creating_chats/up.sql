-- Your SQL goes here
CREATE TABLE chats (
    id uuid DEFAULT gen_random_uuid () NOT NULL UNIQUE PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users,
    chat_id uuid NOT NULL REFERENCES chats,
    content TEXT NOT NULL,
    created TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_connector (
    id SERIAL PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users,
    chat_id uuid NOT NULL REFERENCES chats,
    joined TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO chats (name)
VALUES ('TESZTCHAT');

INSERT INTO chat_connector (user_id, chat_id)
VALUES (
    (SELECT id FROM users WHERE email LIKE 'channelszilu@gmail.com'),
    (SELECT id FROM chats WHERE name LIKE 'TESZTCHAT')
);

INSERT INTO chat_connector (user_id, chat_id)
VALUES (
    (SELECT id FROM users WHERE email LIKE 'nyaltamas@gmail.com'),
    (SELECT id FROM chats WHERE name LIKE 'TESZTCHAT')
);

INSERT INTO messages (user_id, chat_id, content)
VALUES (
    (SELECT id FROM users WHERE email LIKE 'nyaltamas@gmail.com'),
    (SELECT id FROM chats WHERE name LIKE 'TESZTCHAT'),
    'TESZT ÜZI FROM NYAL TAMÁS...'
);

INSERT INTO messages (user_id, chat_id, content)
VALUES (
    (SELECT id FROM users WHERE email LIKE 'channelszilu@gmail.com'),
    (SELECT id FROM chats WHERE name LIKE 'TESZTCHAT'),
    'TESZT ÜZI FROM SZILU...'
);
