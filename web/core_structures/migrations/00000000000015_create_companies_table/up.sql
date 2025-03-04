CREATE TABLE IF NOT EXISTS companies(
    id SERIAL PRIMARY KEY,
    company TEXT NOT NULL UNIQUE CHECK (must_not_be_empty(company)),
    created_by INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (updated_by) REFERENCES users(id)
);