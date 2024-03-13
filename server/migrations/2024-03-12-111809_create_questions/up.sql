-- Your SQL goes here

-- todo: Make uniq constraint for (quiz_id, question_rank)

CREATE TABLE questions (
  id VARCHAR PRIMARY KEY,
  quiz_id VARCHAR NOT NULL REFERENCES quiz(id) ON DELETE CASCADE,
  question VARCHAR NOT NULL,
  question_rank INTEGER NOT NULL,
  max_time REAL NOT NULL,
  max_points REAL NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
	BEFORE UPDATE ON questions
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();