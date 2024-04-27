-- This is postgresql
CREATE TABLE IF NOT EXISTS sessions (
  id UUID PRIMARY KEY default gen_random_uuid(),
  user_id INTEGER NOT NULL,
  token TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id
    FOREIGN KEY(user_id)
    REFERENCES users(id)
);
