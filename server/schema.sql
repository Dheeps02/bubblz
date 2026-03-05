CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS rooms (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY NOT NULL,
    message_type TEXT NOT NULL CHECK(message_type IN ('Text', 'System')),
    sender INTEGER NOT NULL REFERENCES users(id),
    room INTEGER NOT NULL REFERENCES rooms(id),
    timestamp INTEGER NOT NULL,
    content TEXT NOT NULL
);
