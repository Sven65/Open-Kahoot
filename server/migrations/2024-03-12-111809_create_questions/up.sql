-- Your SQL goes here

-- todo: Make uniq constraint for (quiz_id, question_rank)

CREATE TABLE questions (
  id SERIAL PRIMARY KEY,
  quiz_id INTEGER NOT NULL REFERENCES quiz(id),
  question VARCHAR NOT NULL,
  question_rank INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
	BEFORE UPDATE ON questions
	FOR EACH ROW
	EXECUTE PROCEDURE trigger_set_timestamp();