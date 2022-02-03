-- Add up migration script here
CREATE TABLE queue (
	id SERIAL PRIMARY KEY,
	owner TEXT NOT NULL,
	locked BOOL NOT NULL,
	content JSON
)
