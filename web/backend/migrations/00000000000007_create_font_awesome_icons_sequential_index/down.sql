-- Drop sequence ownership and default constraint
ALTER SEQUENCE font_awesome_icons_id_seq OWNED BY NONE;
ALTER TABLE font_awesome_icons ALTER COLUMN id DROP DEFAULT;

-- Drop sequence
DROP SEQUENCE font_awesome_icons_id_seq;