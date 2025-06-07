CREATE TABLE IF NOT EXISTS commercial_products (
    id UUID PRIMARY KEY REFERENCES trackables(id),
	deprecation_date TIMESTAMP WITH TIME ZONE,
	brand_id INTEGER NOT NULL REFERENCES brands(id)
);

CREATE TABLE IF NOT EXISTS instrument_models (
	id UUID PRIMARY KEY REFERENCES commercial_products(id)
);

CREATE TABLE IF NOT EXISTS weighing_instrument_models (
	id UUID PRIMARY KEY REFERENCES instrument_models(id),
	error_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(error_kilograms)),
	minimum_measurable_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(minimum_measurable_kilograms)),
	maximum_measurable_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(maximum_measurable_kilograms)),
	CHECK (must_be_strictly_smaller_than_f32(error_kilograms, minimum_measurable_kilograms)),
	CHECK (must_be_strictly_smaller_than_f32(minimum_measurable_kilograms, maximum_measurable_kilograms))
);

CREATE TABLE IF NOT EXISTS aliquoting_instrument_models (
	id UUID PRIMARY KEY REFERENCES instrument_models(id),
	error_liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(error_liters)),
	minimum_measurable_liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(minimum_measurable_liters)),
	maximum_measurable_liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(maximum_measurable_liters)),
	CHECK (must_be_strictly_smaller_than_f32(error_liters, minimum_measurable_liters)),
	CHECK (must_be_strictly_smaller_than_f32(minimum_measurable_liters, maximum_measurable_liters))
);

CREATE TABLE IF NOT EXISTS container_models (
	id UUID PRIMARY KEY REFERENCES commercial_products(id),
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	container_category ContainerCategory NOT NULL
);

CREATE TABLE IF NOT EXISTS packaging_models (
	id UUID PRIMARY KEY REFERENCES commercial_products(id),
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms))
);
