CREATE TABLE IF NOT EXISTS constellation (
    id TEXT PRIMARY KEY,
    initial TIMESTAMP,
    pos_x REAL,
    pos_y REAL,
    pos_z REAL,
    vel_x REAL,
    vel_y REAL,
    vel_z REAL
);
