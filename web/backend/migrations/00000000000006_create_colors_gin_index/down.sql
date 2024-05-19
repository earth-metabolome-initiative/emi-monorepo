-- Down version of colors_name_trgm_idx
DROP INDEX IF EXISTS colors_name_trgm_idx;

DROP FUNCTION f_concat_colors_name(text, text);