CREATE TABLE IF NOT EXISTS trackables (id SERIAL PRIMARY KEY);
CREATE TABLE IF NOT EXISTS weighing_devices (id INTEGER PRIMARY KEY REFERENCES trackables(id));
CREATE TABLE IF NOT EXISTS procedure_models (id SERIAL PRIMARY KEY);
CREATE TABLE IF NOT EXISTS trackable_procedure_models (
    id SERIAL PRIMARY KEY,
    trackable_id INTEGER NOT NULL REFERENCES trackables(id),
    procedure_model_id INTEGER NOT NULL REFERENCES procedure_models(id),
    UNIQUE (id, trackable_id),
    UNIQUE (id, procedure_model_id)
);
CREATE TABLE IF NOT EXISTS weighing_procedure_models (
    procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
    weighed_with INTEGER NOT NULL REFERENCES trackables(id),
    procedure_weighed_with INTEGER NOT NULL REFERENCES trackable_procedure_models(id),
    reweighed_with INTEGER NOT NULL REFERENCES trackables(id),
    procedure_reweighed_with INTEGER NOT NULL REFERENCES trackable_procedure_models(id),
    FOREIGN KEY (procedure_weighed_with, weighed_with) REFERENCES trackable_procedure_models (id, trackable_id) ON DELETE CASCADE,
    FOREIGN KEY (procedure_weighed_with, procedure_model_id) REFERENCES trackable_procedure_models (id, procedure_model_id) ON DELETE CASCADE,
    FOREIGN KEY (procedure_reweighed_with, reweighed_with) REFERENCES trackable_procedure_models (id, trackable_id) ON DELETE CASCADE,
    FOREIGN KEY (procedure_reweighed_with, procedure_model_id) REFERENCES trackable_procedure_models (id, procedure_model_id) ON DELETE CASCADE,
    UNIQUE (procedure_model_id, weighed_with)
);
CREATE TABLE IF NOT EXISTS specialized_weighing_procedure_models (
    procedure_model_id INTEGER PRIMARY KEY REFERENCES weighing_procedure_models(procedure_model_id),
    specialized_weighed_with INTEGER NOT NULL REFERENCES weighing_devices(id),
    FOREIGN KEY (procedure_model_id, specialized_weighed_with) REFERENCES weighing_procedure_models (procedure_model_id, weighed_with) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS freezing_procedure_models (
    procedure_model_id INTEGER PRIMARY KEY REFERENCES procedure_models(id),
    frozen_with INTEGER NOT NULL REFERENCES trackables(id),
    procedure_frozen_with INTEGER NOT NULL REFERENCES trackable_procedure_models(id),
    FOREIGN KEY (procedure_frozen_with, frozen_with) REFERENCES trackable_procedure_models (id, trackable_id) ON DELETE CASCADE,
    FOREIGN KEY (procedure_frozen_with, procedure_model_id) REFERENCES trackable_procedure_models (id, procedure_model_id) ON DELETE CASCADE
);