CREATE TABLE IF NOT EXISTS roots (
	id SERIAL PRIMARY KEY,
	model_id INTEGER REFERENCES roots(id) ON DELETE CASCADE,
	UNIQUE (id, model_id)
);
CREATE TABLE IF NOT EXISTS children (
	id INTEGER PRIMARY KEY REFERENCES roots(id) ON DELETE CASCADE,
	child_model_id INTEGER REFERENCES children(id) ON DELETE CASCADE,
	FOREIGN KEY (id, child_model_id) REFERENCES roots(id, model_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS grandchildren (
	id INTEGER PRIMARY KEY REFERENCES children(id) ON DELETE CASCADE,
	grandchild_model_id INTEGER REFERENCES grandchildren(id) ON DELETE CASCADE,
	FOREIGN KEY (id, grandchild_model_id) REFERENCES roots(id, model_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS cousins (
	id INTEGER PRIMARY KEY REFERENCES children(id) ON DELETE CASCADE,
	grandchild_model_id INTEGER REFERENCES cousins(id) ON DELETE CASCADE,
	FOREIGN KEY (id, grandchild_model_id) REFERENCES roots(id, model_id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS great_grandchildren (
	id INTEGER PRIMARY KEY REFERENCES grandchildren(id) ON DELETE CASCADE,
	great_grandchild_model_id INTEGER REFERENCES great_grandchildren(id) ON DELETE CASCADE,
	FOREIGN KEY (id, great_grandchild_model_id) REFERENCES roots(id, model_id) ON DELETE CASCADE
);