CREATE TABLE draft (
    user_id BIGINT PRIMARY KEY,
    text TEXT NOT NULL,
    updated_at TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
);