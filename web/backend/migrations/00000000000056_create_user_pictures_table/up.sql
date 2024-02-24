-- Your SQL goes here
CREATE TABLE user_pictures (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    document_id BIGINT NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, document_id)
);
