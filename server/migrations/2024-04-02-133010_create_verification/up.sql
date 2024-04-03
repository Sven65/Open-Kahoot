-- Your SQL goes here

CREATE TABLE email_verification (
  id VARCHAR PRIMARY KEY,
  user_id VARCHAR NOT NULL REFERENCES users(id),
  verification_token VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


CREATE TRIGGER set_timestamp
	BEFORE UPDATE ON email_verification
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();