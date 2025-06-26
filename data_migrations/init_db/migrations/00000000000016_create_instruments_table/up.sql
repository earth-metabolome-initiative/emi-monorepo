CREATE TABLE
	IF NOT EXISTS instrument_models (
		id UUID PRIMARY KEY REFERENCES trackables (id) ON DELETE CASCADE
	);

CREATE TABLE
	IF NOT EXISTS weighing_instrument_models (
		id UUID PRIMARY KEY REFERENCES instrument_models (id) ON DELETE CASCADE
	);

CREATE TABLE
	IF NOT EXISTS positioning_device_models (
		id UUID PRIMARY KEY REFERENCES instrument_models (id) ON DELETE CASCADE
	);

CREATE TABLE
	IF NOT EXISTS camera_models (
		id UUID PRIMARY KEY REFERENCES instrument_models (id) ON DELETE CASCADE
	);

CREATE TABLE
	IF NOT EXISTS pipette_models (
		id UUID PRIMARY KEY REFERENCES instrument_models (id) ON DELETE CASCADE
	);

CREATE TABLE
	IF NOT EXISTS pipette_tip_models (
		id UUID PRIMARY KEY REFERENCES trackables (id) ON DELETE CASCADE
	);

-- CREATE TABLE
-- 	IF NOT EXISTS phone_models (
-- 		id UUID PRIMARY KEY,
-- 		FOREIGN KEY (id) REFERENCES camera_models (id) ON DELETE CASCADE,
-- 		FOREIGN KEY (id) REFERENCES positioning_device_models (id) ON DELETE CASCADE
-- 	);

CREATE TABLE
	IF NOT EXISTS freezer_models (
		id UUID PRIMARY KEY,
		FOREIGN KEY (id) REFERENCES instrument_models (id) ON DELETE CASCADE
	);

CREATE TABLE
	IF NOT EXISTS freeze_drier_models (
		id UUID PRIMARY KEY,
		FOREIGN KEY (id) REFERENCES instrument_models (id) ON DELETE CASCADE
	);

CREATE TABLE
	IF NOT EXISTS centrifuge_models (
		id UUID PRIMARY KEY,
		FOREIGN KEY (id) REFERENCES instrument_models (id) ON DELETE CASCADE
	);

CREATE TABLE
	IF NOT EXISTS ball_mill_machine_models (
		id UUID PRIMARY KEY,
		FOREIGN KEY (id) REFERENCES instrument_models (id) ON DELETE CASCADE
	);
