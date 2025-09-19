//! Submodule providing a wrapper over the `directus_codegen` generated `FieldDatum`
//! struct.

use directus_codegen::FieldDatum;

#[derive(Debug)]
pub(crate) struct FieldDatumWrapper(FieldDatum);

impl AsRef<FieldDatum> for FieldDatumWrapper {
    fn as_ref(&self) -> &FieldDatum {
        &self.0
    }
}

impl From<FieldDatum> for FieldDatumWrapper {
    fn from(value: FieldDatum) -> Self {
        Self(value)
    }
}
