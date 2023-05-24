CREATE TABLE IF NOT EXISTS "Categories" (
	"category_uuid"	TEXT NOT NULL UNIQUE,
	"name"	TEXT,
	"created_at"	TEXT,
	"updated_at"	TEXT,
	PRIMARY KEY("category_uuid")
);

CREATE TABLE IF NOT EXISTS "Comments" (
	"comment_uuid"	TEXT NOT NULL UNIQUE,
	"post_uuid"	TEXT,
	"user_google_uuid"	TEXT,
	"content"	TEXT,
	"created_at"	TEXT,
	"updated_at"	TEXT,
	FOREIGN KEY("post_uuid") REFERENCES "Posts"("post_uuid"),
	FOREIGN KEY("user_google_uuid") REFERENCES "Users"("google_uuid"),
	PRIMARY KEY("comment_uuid")
);

CREATE TABLE IF NOT EXISTS "Likes" (
	"like_uuid"	TEXT NOT NULL UNIQUE,
	"post_uuid"	TEXT,
	"user_google_uuid"	TEXT,
	"created_at"	TEXT,
	FOREIGN KEY("post_uuid") REFERENCES "Posts"("post_uuid"),
	FOREIGN KEY("user_google_uuid") REFERENCES "Users"("google_uuid"),
	PRIMARY KEY("like_uuid")
);

CREATE TABLE IF NOT EXISTS "Post_Category" (
	"post_uuid"	INTEGER,
	"category_uuid"	INTEGER,
	FOREIGN KEY("post_uuid") REFERENCES "Posts"("post_uuid"),
	FOREIGN KEY("category_uuid") REFERENCES "Categories"("category_uuid")
);

CREATE TABLE IF NOT EXISTS "Posts" (
	"post_uuid"	TEXT NOT NULL UNIQUE,
	"user_google_uuid"	TEXT,
	"series_uuid"	TEXT,
	"title"	BLOB NOT NULL,
	"content"	TEXT NOT NULL,
	"created_at"	TEXT,
	"updated_at"	TEXT,
	"draft_saved"	INTEGER NOT NULL CHECK("draft_saved" IN (0, 1)),
	"posted"	INTEGER NOT NULL CHECK("posted" IN (0, 1)),
	FOREIGN KEY("user_google_uuid") REFERENCES "Users"("google_uuid"),
	FOREIGN KEY("series_uuid") REFERENCES "Series"("series_uuid"),
	PRIMARY KEY("post_uuid")
);

CREATE TABLE IF NOT EXISTS "Series" (
	"series_uuid"	TEXT NOT NULL UNIQUE,
	"user_google_uuid"	TEXT,
	"name"	TEXT,
	"description"	TEXT,
	"created_at"	TEXT,
	"updated_at"	TEXT,
	FOREIGN KEY("user_google_uuid") REFERENCES "Users"("google_uuid"),
	PRIMARY KEY("series_uuid")
);

CREATE TABLE IF NOT EXISTS "Users" (
	"google_uuid"	TEXT NOT NULL UNIQUE,
	"first_name"	TEXT,
	"last_name"	TEXT,
	"email"	TEXT,
	"created_at"	TEXT,
	"updated_at"	TEXT,
	PRIMARY KEY("google_uuid")
);
