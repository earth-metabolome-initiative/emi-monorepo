//! Submodule defining methods to determine if a FieldDatum should be skipped.

use crate::structs::FieldDatumWrapper;


impl FieldDatumWrapper {
    /// Determines if the FieldDatum should be skipped during migration.
    /// A FieldDatum should be skipped if it lacks a sample source or if its
    /// sample source kind is unsupported.
    pub fn should_skip(&self) -> bool {
        if self.as_ref().sample_id.starts_with("obs_") {
            return true;
        }


        false
    }
}