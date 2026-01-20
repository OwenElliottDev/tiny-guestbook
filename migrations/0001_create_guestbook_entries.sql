CREATE TABLE guestbook_entries (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT,
    rating INTEGER NOT NULL,
    note TEXT NOT NULL,
    posted_at_utc INTEGER NOT NULL
);
