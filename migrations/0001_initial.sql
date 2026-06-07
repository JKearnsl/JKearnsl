CREATE TABLE IF NOT EXISTS notes (
    id CHAR(16) PRIMARY KEY,
    no INTEGER NOT NULL,
    slug VARCHAR(64) UNIQUE NOT NULL,
    category TEXT NOT NULL CHECK(category IN ('prog', 'math', 'science')),
    title VARCHAR(128) NOT NULL,
    description VARCHAR(256) NOT NULL,
    body VARCHAR(32768) NOT NULL,
    featured INTEGER NOT NULL CHECK(featured IN (0, 1)),
    state TEXT NOT NULL CHECK(state IN ('Published', 'Draft')),
    created_at INTEGER NOT NULL,
    updated_at INTEGER
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS tags (
    id CHAR(16) PRIMARY KEY,
    name VARCHAR(64) UNIQUE NOT NULL
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS note_tags (
    note_id CHAR(16) NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
    tag_id  CHAR(16) NOT NULL REFERENCES tags(id)  ON DELETE CASCADE,
    PRIMARY KEY (note_id, tag_id)
) WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS users (
    id CHAR(16) PRIMARY KEY,
    username VARCHAR(128) UNIQUE NOT NULL,
    password_phc TEXT NOT NULL,
    created_at INTEGER NOT NULL
) WITHOUT ROWID;

CREATE TABLE sessions (
    token_hash BLOB PRIMARY KEY,
    user_id CHAR(16) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL
) WITHOUT ROWID;


CREATE TABLE IF NOT EXISTS projects (
    id CHAR(16) PRIMARY KEY,
    title VARCHAR(128) NOT NULL,
    description VARCHAR(256) NOT NULL,
    url VARCHAR(2048),
    created_at INTEGER NOT NULL
) WITHOUT ROWID;
