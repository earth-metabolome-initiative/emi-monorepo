CREATE TABLE IF NOT EXISTS commercial_products (
    id SERIAL PRIMARY KEY,
	name TEXT NOT NULL UNIQUE CHECK (must_be_paragraph(name)),
	description TEXT NOT NULL CHECK (must_be_paragraph(description)),
	photograph_id UUID NOT NULL REFERENCES documents(id),
	deprecation_date TIMESTAMP WITH TIME ZONE,
	brand_id INTEGER NOT NULL REFERENCES brands(id),
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS instrument_models (
	id INTEGER PRIMARY KEY REFERENCES commercial_products(id),
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS weighing_instrument_models (
	id INTEGER PRIMARY KEY REFERENCES instrument_models(id),
	error_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(error_kilograms)),
	minimum_measurable_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(minimum_measurable_kilograms)),
	maximum_measurable_kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(maximum_measurable_kilograms)),
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_strictly_smaller_than_f32(error_kilograms, minimum_measurable_kilograms)),
	CHECK (must_be_strictly_smaller_than_f32(minimum_measurable_kilograms, maximum_measurable_kilograms))
);

CREATE TABLE IF NOT EXISTS aliquoting_instrument_models (
	id INTEGER PRIMARY KEY REFERENCES instrument_models(id),
	error_liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(error_liters)),
	minimum_measurable_liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(minimum_measurable_liters)),
	maximum_measurable_liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(maximum_measurable_liters)),
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CHECK (must_be_strictly_smaller_than_f32(error_liters, minimum_measurable_liters)),
	CHECK (must_be_strictly_smaller_than_f32(minimum_measurable_liters, maximum_measurable_liters))
);

CREATE TABLE IF NOT EXISTS instrument_model_categories (
	id SERIAL PRIMARY KEY,
	instrument_model_id INTEGER NOT NULL REFERENCES instrument_models(id),
	instrument_category InstrumentCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS container_models (
	id INTEGER PRIMARY KEY REFERENCES commercial_products(id),
	liters REAL NOT NULL CHECK (must_be_strictly_positive_f32(liters)),
	container_category ContainerCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS packaging_models (
	id INTEGER PRIMARY KEY REFERENCES commercial_products(id),
	kilograms REAL NOT NULL CHECK (must_be_strictly_positive_f32(kilograms)),
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tool_models (
	id INTEGER PRIMARY KEY REFERENCES commercial_products(id),
	tool_category ToolCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS nameplate_models (
	id INTEGER PRIMARY KEY REFERENCES commercial_products(id),
	nameplate_category NameplateCategory NOT NULL,
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS commercial_reagent_models (
	id INTEGER PRIMARY KEY REFERENCES commercial_products(id),
	reagent_id INTEGER NOT NULL REFERENCES reagents(id) ON DELETE CASCADE,
	created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
