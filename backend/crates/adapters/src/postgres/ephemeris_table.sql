CREATE TABLE "ephemeris" (
  id text NOT NULL,
  datetime timestamp without time zone NOT NULL,
  pos_x double precision NOT NULL,
  pos_y double precision NOT NULL,
  pos_z double precision NOT NULL,
  vel_x double precision NOT NULL,
  vel_y double precision NOT NULL,
  vel_z double precision NOT NULL,
  sma double precision,
  ecc double precision,
  inc double precision,
  raan double precision,
  arg double precision,
  ta double precision,
  reference_frame text NOT NULL,
  source text NOT NULL,
  PRIMARY KEY (id, datetime)
);
