CREATE INDEX users_gin ON users USING gin (
  username gin_trgm_ops
);

CREATE INDEX composite_users_gist ON composite_users USING gist (
  username gist_trgm_ops
);