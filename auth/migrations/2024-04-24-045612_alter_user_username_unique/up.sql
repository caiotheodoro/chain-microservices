-- Your SQL goes here
ALTER TABLE users
ADD CONSTRAINT users_username_unique UNIQUE (username);