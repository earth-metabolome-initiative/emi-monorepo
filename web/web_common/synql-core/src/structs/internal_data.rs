//! Submodule providing a struct which defines a data model.

use std::rc::Rc;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use crate::structs::{
    ExternalCrate, InternalCrate, InternalEnum, InternalModule, InternalStruct, Publicness,
    external_crate::ExternalTypeRef,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enum representing the variant of internal data (struct or enum).
pub enum InternalDataVariant<'data> {
    /// Variant representing a struct.
    StructVariant(InternalStruct<'data>),
    /// Variant representing an enum.
    EnumVariant(InternalEnum<'data>),
}

impl<'data> InternalDataVariant<'data> {
    /// Returns the internal crate dependencies of the variant.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        match self {
            InternalDataVariant::StructVariant(s) => s.internal_dependencies(),
            InternalDataVariant::EnumVariant(e) => e.internal_dependencies(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Enum representing a variant of internal data, which may be defined within
/// the workspace or come from an external crate.
pub enum DataVariantRef<'data> {
    /// Variant representing internal data defined within the workspace.
    Internal(InternalDataRef<'data>),
    /// Variant representing data defined within an external crate.
    External(ExternalTypeRef<'data>),
}

impl<'data> DataVariantRef<'data> {
    /// Returns the internal crate dependencies of the variant.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        match self {
            DataVariantRef::Internal(internal) => internal.internal_dependencies(),
            DataVariantRef::External(_) => vec![],
        }
    }

    /// Returns the external crate dependencies of the variant.
    pub fn external_dependencies(&self) -> Vec<&'data ExternalCrate> {
        match self {
            DataVariantRef::Internal(_) => vec![],
            DataVariantRef::External(external) => vec![external.external_crate()],
        }
    }
}

#[derive(Debug, Clone)]
/// Enum representing a trait implemented for some internal data,
/// which may be defined within the workspace or come from an external crate.
pub enum TraitVariantRef<'data> {
    /// Variant representing a trait defined within the workspace.
    Internal(TokenStream, &'data InternalCrate<'data>),
    /// Variant representing a trait defined within an external crate.
    External(TokenStream, &'data ExternalCrate),
}

impl<'data> PartialEq for TraitVariantRef<'data> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                TraitVariantRef::Internal(stream_a, krate_a),
                TraitVariantRef::Internal(stream_b, krate_b),
            ) => krate_a == krate_b && stream_a.to_string() == stream_b.to_string(),
            (
                TraitVariantRef::External(stream_a, crate_a),
                TraitVariantRef::External(stream_b, crate_b),
            ) => crate_a == crate_b && stream_a.to_string() == stream_b.to_string(),
            _ => false,
        }
    }
}

impl Eq for TraitVariantRef<'_> {}

impl PartialOrd for TraitVariantRef<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TraitVariantRef<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (
                TraitVariantRef::Internal(stream_a, krate_a),
                TraitVariantRef::Internal(stream_b, krate_b),
            ) => {
                let crate_cmp = krate_a.name().cmp(krate_b.name());
                if crate_cmp != std::cmp::Ordering::Equal {
                    return crate_cmp;
                }
                stream_a.to_string().cmp(&stream_b.to_string())
            }
            (
                TraitVariantRef::External(stream_a, crate_a),
                TraitVariantRef::External(stream_b, crate_b),
            ) => {
                let crate_cmp = crate_a.name().cmp(crate_b.name());
                if crate_cmp != std::cmp::Ordering::Equal {
                    return crate_cmp;
                }
                stream_a.to_string().cmp(&stream_b.to_string())
            }
            (TraitVariantRef::Internal(_, _), TraitVariantRef::External(_, _)) => {
                std::cmp::Ordering::Less
            }
            (TraitVariantRef::External(_, _), TraitVariantRef::Internal(_, _)) => {
                std::cmp::Ordering::Greater
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct representing a reference to internal data and its crate.
pub struct InternalDataRef<'data> {
    data: Rc<InternalData<'data>>,
    internal_crate: Rc<InternalCrate<'data>>,
}

impl<'data> InternalDataRef<'data> {
    pub(crate) fn new(
        internal_crate: &Rc<InternalCrate<'data>>,
        data: &Rc<InternalData<'data>>,
    ) -> InternalDataRef<'data> {
        InternalDataRef { data: data.clone(), internal_crate: internal_crate.clone() }
    }

    /// Returns the internal data.
    pub fn data(&self) -> &InternalData<'data> {
        self.data.as_ref()
    }

    /// Returns the internal crate.
    pub fn internal_crate(&self) -> &InternalCrate<'data> {
        self.internal_crate.as_ref()
    }

    /// Returns the name of the internal data.
    pub fn name(&self) -> &str {
        self.data.name()
    }

    /// Returns the ident of the internal crate.
    pub fn crate_ident(&self) -> Ident {
        self.internal_crate.ident()
    }

    /// Returns the internal crate dependencies of the variant.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        vec![self.internal_crate.as_ref()]
    }
}

impl ToTokens for DataVariantRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            DataVariantRef::Internal(internal) => {
                let internal_crate_ident = internal.internal_crate.ident();
                let internal_data_ident = internal.data.ident();
                tokens.extend(quote! {#internal_crate_ident::prelude::#internal_data_ident});
            }
            DataVariantRef::External(external) => {
                external.rust_type().to_tokens(tokens);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct representing a reference to internal module and its crate.
pub struct InternalModuleRef<'data> {
    module: Rc<InternalModule<'data>>,
    internal_crate: Rc<InternalCrate<'data>>,
}

impl<'data> InternalModuleRef<'data> {
    /// Creates a new `InternalModuleRef`.
    ///
    /// # Arguments
    ///
    /// * `internal_crate` - A reference to the internal crate.
    /// * `module` - A reference to the internal module.
    pub fn new(
        internal_crate: &Rc<InternalCrate<'data>>,
        module: &Rc<InternalModule<'data>>,
    ) -> InternalModuleRef<'data> {
        InternalModuleRef { module: module.clone(), internal_crate: internal_crate.clone() }
    }

    /// Returns the internal module.
    pub fn module(&self) -> &InternalModule<'data> {
        self.module.as_ref()
    }

    /// Returns the internal crate.
    pub fn internal_crate(&self) -> &InternalCrate<'data> {
        self.internal_crate.as_ref()
    }

    /// Returns the name of the internal module.
    pub fn name(&self) -> &str {
        self.module.name()
    }

    /// Returns the ident of the internal crate.
    pub fn crate_ident(&self) -> Ident {
        self.internal_crate.ident()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Struct defining a data model.
pub struct InternalData<'data> {
    /// Publicness of the data.
    publicness: Publicness,
    /// Name of the data.
    name: String,
    /// The variant of the data (struct or enum).
    variant: InternalDataVariant<'data>,
    /// The traits implemented for the data.
    traits: Vec<TraitVariantRef<'data>>,
}

impl<'data> InternalData<'data> {
    /// Returns a reference to the publicness of the data.
    pub fn publicness(&self) -> &Publicness {
        &self.publicness
    }

    /// Returns whether the data is public.
    pub fn is_public(&self) -> bool {
        self.publicness.is_public()
    }

    /// Returns the name of the data.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the name of the data.
    pub fn ident(&self) -> Ident {
        syn::Ident::new(&self.name, proc_macro2::Span::call_site())
    }

    /// Returns a reference to the variant of the data.
    pub fn variant(&self) -> &InternalDataVariant<'data> {
        &self.variant
    }

    /// Returns the sorted unique internal crate dependencies of the data.
    pub fn internal_dependencies(&self) -> Vec<&'data InternalCrate<'data>> {
        let mut crates = self
            .traits
            .iter()
            .filter_map(|t| {
                if let TraitVariantRef::Internal(_, krate) = t { Some(*krate) } else { None }
            })
            .collect::<Vec<_>>();

        crates.sort_unstable();
        crates.dedup();
        crates
    }

    /// Returns the sorted unique external crate dependencies of the data.
    pub fn external_dependencies(&self) -> Vec<&'data ExternalCrate> {
        let mut crates = self
            .traits
            .iter()
            .filter_map(|t| {
                if let TraitVariantRef::External(_, krate) = t { Some(*krate) } else { None }
            })
            .collect::<Vec<_>>();
        crates.sort_unstable();
        crates.dedup();
        crates
    }
}

impl<'data> ToTokens for InternalData<'data> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let publicness = &self.publicness;
        let ident = self.ident();
        let variant = match &self.variant {
            InternalDataVariant::StructVariant(s) => {
                quote::quote! {
                    struct #ident #s
                }
            }
            InternalDataVariant::EnumVariant(e) => {
                quote::quote! {
                    enum #ident #e
                }
            }
        };
        let token = quote::quote! {
            #publicness #variant
        };
        tokens.extend(token);
    }
}
