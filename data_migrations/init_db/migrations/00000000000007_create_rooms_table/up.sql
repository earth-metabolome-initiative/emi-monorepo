CREATE TABLE IF NOT EXISTS rooms (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL  CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	qrcode UUID NOT NULL UNIQUE,
	addresses_id INTEGER NOT NULL REFERENCES addresses(id),
	geolocation GEOGRAPHY(POINT, 4326) NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (created_at <= updated_at)
);