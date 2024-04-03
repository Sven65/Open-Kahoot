-- Your SQL goes here

ALTER TABLE users
ADD COLUMN verified_email BOOLEAN DEFAULT false;