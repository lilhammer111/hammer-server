CREATE TABLE account (
	id          BIGSERIAL PRIMARY KEY,
	username    VARCHAR(255) UNIQUE NOT NULL,
	password    VARCHAR(255),
	mobile      CHAR(11) UNIQUE,
	email       VARCHAR(255) UNIQUE,
	avatar_url  VARCHAR(255),
	pronouns    VARCHAR(255),
	birthday    DATE,
	industry    VARCHAR(255),
	location    VARCHAR(255),
	social_account TEXT[],
	created_at   TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_at   TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP
);