-- This file should undo anything in `up.sql`

--- files ---

ALTER TABLE files
DROP CONSTRAINT files_owner_id_fkey;

--- email_verification ---

ALTER TABLE email_verification
DROP CONSTRAINT email_verification_user_id_fkey;


--- password_reset ---

ALTER TABLE password_reset
DROP CONSTRAINT password_reset_user_id_fkey;

--- session ---

ALTER TABLE session
DROP CONSTRAINT session_user_id_fkey;
