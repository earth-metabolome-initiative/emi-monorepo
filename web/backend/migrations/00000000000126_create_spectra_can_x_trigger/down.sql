-- Drop the `can_update_spectra` function and trigger on the spectra table.

DROP TRIGGER can_update_spectra ON spectra;
DROP FUNCTION IF EXISTS can_update_spectra_trigger();
DROP FUNCTION IF EXISTS can_update_spectra(INTEGER, INTEGER);
-- Drop the `can_admin_spectra` function and trigger on the spectra table.

DROP FUNCTION IF EXISTS can_admin_spectra(INTEGER, INTEGER);
-- Drop the `can_view_spectra` function and trigger on the spectra table.

DROP FUNCTION IF EXISTS can_view_spectra(INTEGER, INTEGER);
