//! Submodule providing helper methods for internal data structures.

use crate::structs::DataVariantRef;

impl DataVariantRef {
    /// Returns true if the `DataVariantRef` is of boolean type.
    pub fn is_bool(&self) -> bool {
        if let Self::External(external_type) = self { external_type.is_bool() } else { false }
    }

    /// Returns true if the `DataVariantRef` is of numeric type.
    pub fn is_numeric(&self) -> bool {
        if let Self::External(external_type) = self { external_type.is_numeric() } else { false }
    }
}
