CREATE TABLE IF NOT EXISTS emails (
    id UUID PRIMARY KEY default gen_random_uuid(),
    source TEXT NOT NULL,
    subject TEXT NOT NULL,
    contents TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS email_verification_tokens (
    id serial PRIMARY KEY,
    email_id UUID NOT NULL,
    token TEXT NOT NULL,
    CONSTRAINT fk_email FOREIGN KEY (email_id) REFERENCES emails (id)
);
