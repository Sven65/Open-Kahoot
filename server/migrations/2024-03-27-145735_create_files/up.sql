-- Your SQL goes here

DO $$ BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_type
        WHERE typname = 'filehostprovider'
        AND typtype = 'e'
    ) THEN
        CREATE TYPE filehostprovider AS ENUM ('disk', 's3');
    END IF;
END $$;


CREATE TABLE files (
  id VARCHAR PRIMARY KEY,
  owner_id VARCHAR NOT NULL REFERENCES users(id),
  question_id VARCHAR REFERENCES questions(id) ON DELETE CASCADE,
  file_location VARCHAR,
  host filehostprovider NOT NULL,
  has_upload boolean NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
	BEFORE UPDATE ON files
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();