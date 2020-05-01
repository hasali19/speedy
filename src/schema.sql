CREATE TABLE IF NOT EXISTS results (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp   TEXT    NOT NULL,
    ping        REAL    NOT NULL,
    download    INTEGER NOT NULL,
    upload      INTEGER NOT NULL
);
