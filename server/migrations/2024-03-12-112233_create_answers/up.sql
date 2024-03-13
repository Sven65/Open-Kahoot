-- Your SQL goes here

-- todo: Make uniq constraint for (question_id, answer_color)

DO $$ BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_type
        WHERE typname = 'answer_color'
        AND typtype = 'e'
    ) THEN
        CREATE TYPE answer_color AS ENUM ('Red', 'Green', 'Blue', 'Yellow');
    END IF;
END $$;


CREATE TABLE answers (
  id VARCHAR PRIMARY KEY,
  question_id VARCHAR NOT NULL REFERENCES questions(id),
  answer VARCHAR NOT NULL,
  is_correct BOOLEAN NOT NULL,
  answer_color answer_color NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
	BEFORE UPDATE ON answers
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();