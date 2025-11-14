//! Submodule providing helper methods for external crate structures.

use quote::ToTokens;

use crate::structs::{
    ExternalCrate, ExternalTraitRef, ExternalType, ExternalTypeRef, TraitVariantRef,
};

impl ExternalTypeRef {
    /// Returns true if the `ExternalTypeRef` is of boolean type.
    pub fn is_bool(&self) -> bool {
        self.type_ref.is_bool()
    }

    /// Returns true if the `ExternalTypeRef` is of numeric type.
    pub fn is_numeric(&self) -> bool {
        self.type_ref.is_numeric()
    }
}

impl ExternalTraitRef {
    /// Returns the `Sized` trait reference from the `std` crate.
    pub fn sized() -> Self {
        let std_crate = ExternalCrate::std();
        let sized_trait = std_crate
            .external_trait_ref("Sized")
            .expect("Failed to get the `Sized` trait from the `std` crate");
        sized_trait.into()
    }
}

impl TraitVariantRef {
    /// Returns the `Sized` trait variant reference.
    pub fn sized() -> Self {
        ExternalTraitRef::sized().into()
    }
}

impl ExternalType {
    /// Returns true if the `ExternalType` is of boolean type.
    pub fn is_bool(&self) -> bool {
        self.rust_type().to_token_stream().to_string() == "bool"
    }

    /// Returns true if the `ExternalType` is of numeric type.
    pub fn is_numeric(&self) -> bool {
        matches!(
            self.rust_type().to_token_stream().to_string().as_str(),
            "i8" | "i16"
                | "i32"
                | "i64"
                | "i128"
                | "u8"
                | "u16"
                | "u32"
                | "u64"
                | "u128"
                | "f32"
                | "f64"
        )
    }
}
