-- SQLite data types: TEXT, INTEGER, REAL, BLOB
-- See https://www.sqlite.org/datatype3.html

CREATE TABLE users (
	id INTEGER PRIMARY KEY,
	username TEXT NOT NULL,
	password TEXT NOT NULL, -- Hashed, of course
	date_created TEXT NOT NULL, -- An ISO-formatted datetime
);

CREATE TABLE aliases (
	id INTEGER PRIMARY KEY,
	name TEXT,
	owner_id INTEGER, -- The ID of the user that owns this alias
	date_created TEXT,
	profile_description TEXT,
	avatar_filename : TEXT -- If null, then we could use gravatar etc instead?
);

CREATE TABLE posts (
	id INTEGER PRIMARY KEY,
	alias_id INTEGER NOT NULL, -- The id of the alias that this post has been made under
	date_created TEXT,
	date_modified TEXT,
	content TEXT, -- Rendered content (should we support markdown etc in the future)
);

CREATE TABLE attachments (
	id INTEGER PRIMARY KEY,
	post_id INTEGER NOT NULL,
	filename TEXT NOT NULL
)
