//! Submodule defining methods to determine if a FieldDatum should be skipped.

use crate::structs::FieldDatumWrapper;

impl FieldDatumWrapper {
    /// Determines if the FieldDatum should be skipped during migration.
    ///
    /// # Examples
    ///
    /// * A FieldDatum should be skipped if it lacks a sample source or if its
    /// sample source kind is unsupported.
    ///
    /// * Skip entries where collector_fullname and picture_panel are both None
    pub fn should_skip(&self) -> bool {
        if self.as_ref().sample_id.starts_with("obs_") {
            return true;
        }
        if self.as_ref().collector_fullname.is_none() && self.as_ref().picture_panel.is_none() {
            return true;
        }
        false
    }
}
