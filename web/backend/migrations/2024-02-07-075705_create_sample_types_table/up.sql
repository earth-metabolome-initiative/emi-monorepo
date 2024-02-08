-- SQL defining the sample_types table.
-- A sample type is a type of sample that can be collected and stored in a laboratory.
-- This includes also processed samples, such as dissolved samples in ethanol.
-- Generally speaking, each sample that requires some specific treatment or storage
-- conditions should have its own sample type. The sample_types table contains
-- a name column to store the name of the sample type, a description column to store
-- a description of the sample type, the created_at and updated_at columns to store
-- the creation and last update time of the record, plus who created and last updated
-- the record. Some samples may have, even in the best case, a limited lifetime, and
-- the sample type table contains a column to store the lifetime of the sample type.
CREATE TABLE sample_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE,
  description TEXT,
  created_by INT NOT NULL,
  updated_by INT NOT NULL,
  best_before INTERVAL NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (created_by) REFERENCES users (id) ON DELETE CASCADE,
  FOREIGN KEY (updated_by) REFERENCES users (id) ON DELETE CASCADE
);
