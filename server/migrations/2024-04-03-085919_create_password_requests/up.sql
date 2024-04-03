-- Your SQL goes here
CREATE TABLE password_reset (
  id VARCHAR PRIMARY KEY,
  user_id VARCHAR NOT NULL REFERENCES users(id),
  reset_token VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE TRIGGER set_timestamp
	BEFORE UPDATE ON password_reset
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();