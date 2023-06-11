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

CREATE TABLE IF NOT EXISTS user_permissions (
    user_id  INTEGER NOT NULL UNIQUE,
    token    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS series (
	id	INTEGER NOT NULL UNIQUE,
	user_id	INTEGER NOT NULL,
	name	TEXT,
	description	TEXT,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY(user_id) REFERENCES users(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS posts (
	id	INTEGER NOT NULL UNIQUE,
	user_id	INTEGER NOT NULL,
	series_id	INTEGER,
	title	TEXT NOT NULL,
	tagline	TEXT NOT NULL,
	content	TEXT NOT NULL,
	draft_saved	INTEGER NOT NULL CHECK(draft_saved IN (0, 1)),
	posted	INTEGER NOT NULL CHECK(posted IN (0, 1)),
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
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
	post_id	INTEGER NOT NULL UNIQUE,
	user_id	INTEGER NOT NULL UNIQUE,
	content	TEXT,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY(post_id) REFERENCES posts(id),
	FOREIGN KEY(user_id) REFERENCES users(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS likes (
	id	INTEGER NOT NULL UNIQUE,
	post_id	INTEGER NOT NULL UNIQUE,
	user_id	INTEGER NOT NULL UNIQUE,
	created_at	TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY(post_id) REFERENCES posts(id),
	FOREIGN KEY(user_id) REFERENCES users(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

CREATE TABLE IF NOT EXISTS post_category (
	post_id	INTEGER NOT NULL UNIQUE,
	category_id	INTEGER NOT NULL UNIQUE,
	FOREIGN KEY(post_id) REFERENCES posts(id),
	FOREIGN KEY(category_id) REFERENCES categories(id)
);

CREATE TABLE IF NOT EXISTS user_subscription (
	user_id	INTEGER NOT NULL,
	subscription_user_id	INTEGER NOT NULL,
	FOREIGN KEY(user_id) REFERENCES users(id),
	FOREIGN KEY(subscription_user_id) REFERENCES users(id)
);

INSERT INTO users (first_name, last_name, username, password_hash)
	VALUES ('Guest', 'Guestington', 'guest', '');

INSERT INTO users (first_name, last_name, username, password_hash)
	VALUES ('Blake', 'Boris', 'blakeboris', '$2b$12$m8OS9qH0XPvvK5MVTuuhMuqHKKw4.rvp53JgUGDvW/ICHn2mWDAMG');

INSERT INTO series (user_id, name, description)
	VALUES (2, 'How To', 'A Series of Tutorials');

INSERT INTO posts (user_id, series_id, title, tagline, content, draft_saved, posted)
	VALUES (2, 1, 'How to Tie a Tie', 
			'This invigorating article walks you through the trials and tribulations of tying a tie!', 
			"1. Drape the tie around your neck. With your collar up and your shirt fully buttoned, place the tie around your shoulders. Hang the wider end of the tie on your right side, with the narrow end about 12 inches (30 cm) higher on the left.[1]
2. Cross the wide end over the narrow end. Bring the wide end to the left side of your body, over the narrow end. Hold the two pieces of cloth together with your left hand, near your neck.
3. Loop the wide end under the narrow end. Let go with your right hand. Tuck it underneath the narrow end, grab the wide end, and pull it back through to your right side.
4. Loop the wide end back over again. Cross it over the narrow end one more time, at the same point where your left hand is holding the knot together.
5. Pull the wide end up through the neck loop. Fold the tip of the wide end under itself and pull up through the neck loop.
6. Insert the wide end down through the front knot. You should have a horizontal knot across the front of your tie. Hold this knot open with your finger and carefully insert the wide end.
7.  Tighten the knot. Hold the narrow end and slide the front knot up to tighten the tie. Make sure your tie is straight and the length is appropriate, ideally ending at the top of your belt buckle.
	- Squeeze the sides of the knot gently to create a dimple just below it.
8. Tuck the narrow end of the tie into the loop on the back side of the wide end.
9. Fold your collar down, and make sure that the tie is covered by the collar all the way around your neck.
", 1, 0);