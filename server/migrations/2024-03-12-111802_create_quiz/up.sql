-- Your SQL goes here

CREATE TABLE quiz (
  id VARCHAR PRIMARY KEY,
  owner_id VARCHAR NOT NULL REFERENCES users(id),
  name VARCHAR NOT NULL,
  public BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
	BEFORE UPDATE ON quiz
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();