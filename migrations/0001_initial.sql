CREATE TABLE IF NOT EXISTS notes (
    id TEXT PRIMARY KEY,
    no INTEGER NOT NULL DEFAULT 0,
    slug TEXT UNIQUE NOT NULL,
    category TEXT NOT NULL DEFAULT 'prog',
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    body TEXT NOT NULL DEFAULT '',
    featured INTEGER NOT NULL DEFAULT 0,
    state TEXT NOT NULL DEFAULT 'Draft',
    created_at TEXT NOT NULL,
    updated_at TEXT
);

CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS note_tags (
    note_id TEXT NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    tag_id  TEXT NOT NULL REFERENCES tags(id)  ON DELETE CASCADE,
    PRIMARY KEY (note_id, tag_id)
);

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_phc TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE sessions (
    token_hash BLOB PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL
);


CREATE TABLE IF NOT EXISTS projects (
    id CHAR(16) PRIMARY KEY,
    title VARCHAR(128) NOT NULL,
    description VARCHAR(256) NOT NULL,
    url VARCHAR(2048),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);
