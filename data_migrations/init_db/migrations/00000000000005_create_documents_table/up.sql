CREATE TABLE IF NOT EXISTS documents (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	mime_type MediaType NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_smaller_than_utc(created_at, updated_at))
);