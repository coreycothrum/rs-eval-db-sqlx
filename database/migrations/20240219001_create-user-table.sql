CREATE TABLE IF NOT EXISTS user (
  id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  username  TEXT    UNIQUE                    NOT NULL,
  email     TEXT    UNIQUE                    NOT NULL
);
