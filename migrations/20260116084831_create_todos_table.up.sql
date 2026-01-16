-- Add up migration script here
--

CREATE TABLE IF NOT EXISTS todos(
  id SERIAL PRIMARY KEY NOT NULL,
  description TEXT NOT NULL,
  is_complete BOOLEAN NOT NULL DEFAULT false,
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

