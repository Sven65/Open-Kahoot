-- Your SQL goes here

--- files ---

ALTER TABLE files
DROP CONSTRAINT IF EXISTS files_owner_id_fkey;

ALTER TABLE files
ADD CONSTRAINT files_owner_id_fkey
FOREIGN KEY (owner_id) REFERENCES users(id)
ON DELETE CASCADE;

--- email_verification ---

ALTER TABLE email_verification
DROP CONSTRAINT IF EXISTS email_verification_user_id_fkey;

ALTER TABLE email_verification
ADD CONSTRAINT email_verification_user_id_fkey
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;

--- password_reset ---

ALTER TABLE password_reset
DROP CONSTRAINT IF EXISTS password_reset_user_id_fkey;

ALTER TABLE password_reset
ADD CONSTRAINT password_reset_user_id_fkey
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;

--- session ---

ALTER TABLE session
DROP CONSTRAINT IF EXISTS session_user_id_fkey;

ALTER TABLE session
ADD CONSTRAINT session_user_id_fkey
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE;
