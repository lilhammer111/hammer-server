CREATE TABLE comment (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT  NOT NULL,
    parent      BIGINT NOT NULL,
    content     TEXT NOT NULL,
    created_at  TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
)