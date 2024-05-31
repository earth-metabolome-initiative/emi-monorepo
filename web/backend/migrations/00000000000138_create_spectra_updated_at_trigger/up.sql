-- Create the `spectra_updated_at_trigger` trigger on the spectra table.

CREATE FUNCTION spectra_updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER spectra_updated_at_trigger
BEFORE UPDATE ON spectra
FOR EACH ROW
EXECUTE FUNCTION spectra_updated_at_trigger();
