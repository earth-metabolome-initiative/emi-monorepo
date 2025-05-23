CREATE TABLE IF NOT EXISTS login_providers (
  id SMALLSERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL UNIQUE CHECK (must_be_paragraph(name)),
  icon TEXT NOT NULL CHECK (must_be_font_awesome_class(icon)),
  client_id VARCHAR(255) NOT NULL  CHECK (must_be_paragraph(client_id)),
  redirect_uri VARCHAR(255) NOT NULL,
  oauth_url VARCHAR(255) NOT NULL,
  scope VARCHAR(255) NOT NULL CHECK (must_be_paragraph(scope))
);
