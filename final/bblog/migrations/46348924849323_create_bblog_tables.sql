CREATE TABLE IF NOT EXISTS users (
	id	INTEGER NOT NULL UNIQUE,
	first_name	TEXT,
	last_name	TEXT,
	username	TEXT,
	password_hash	TEXT,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY(id AUTOINCREMENT)
);

INSERT INTO users (first_name, last_name, username, password_hash)
	VALUES ('Guest', 'Guestington', 'guest', '');

CREATE TABLE IF NOT EXISTS user_permissions (
    user_id  INTEGER NOT NULL,
    token    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS series (
	id	INTEGER NOT NULL UNIQUE,
	user_id	TEXT,
	name	TEXT,
	description	TEXT,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY(user_id) REFERENCES users(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS posts (
	id	INTEGER NOT NULL UNIQUE,
	user_id	TEXT,
	series_id	TEXT,
	title	TEXT NOT NULL,
	content	TEXT NOT NULL,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	draft_saved	INTEGER NOT NULL CHECK(draft_saved IN (0, 1)),
	posted	INTEGER NOT NULL CHECK(posted IN (0, 1)),
	FOREIGN KEY(user_id) REFERENCES users(id),
	FOREIGN KEY(series_id) REFERENCES series(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS categories (
	id	INTEGER NOT NULL UNIQUE,
	name	TEXT,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS comments (
	id	INTEGER NOT NULL UNIQUE,
	post_id	TEXT,
	user_id	TEXT,
	content	TEXT,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY(post_id) REFERENCES posts(id),
	FOREIGN KEY(user_id) REFERENCES users(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS likes (
	id	INTEGER NOT NULL UNIQUE,
	post_id	TEXT,
	user_id	TEXT,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY(post_id) REFERENCES posts(id),
	FOREIGN KEY(user_id) REFERENCES users(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS post_category (
	post_id	INTEGER,
	category_id	INTEGER,
	FOREIGN KEY(post_id) REFERENCES posts(id),
	FOREIGN KEY(category_id) REFERENCES categories(id)
);
