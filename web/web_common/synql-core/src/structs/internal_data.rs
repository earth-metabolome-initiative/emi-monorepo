//! Submodule providing a struct which defines a data model.

mod builder;

use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub use builder::InternalDataBuilder;
use quote::{ToTokens, quote};
use syn::{Ident, Lifetime};

use crate::{
    structs::{
        Decorator, Derive, ExternalCrate, InternalCrate, InternalEnum, InternalModule,
        InternalStruct, Publicness, external_crate::ExternalTypeRef,
        external_trait::TraitVariantRef,
    },
    traits::{ExternalDependencies, InternalDependencies},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Enum representing the variant of internal data (struct or enum).
pub enum InternalDataVariant<'data> {
    /// Variant representing a struct.
    StructVariant(InternalStruct<'data>),
    /// Variant representing an enum.
    EnumVariant(InternalEnum<'data>),
}

impl<'data> From<InternalStruct<'data>> for InternalDataVariant<'data> {
    fn from(struct_variant: InternalStruct<'data>) -> Self {
        InternalDataVariant::StructVariant(struct_variant)
    }
}

impl<'data> From<InternalEnum<'data>> for InternalDataVariant<'data> {
    fn from(enum_variant: InternalEnum<'data>) -> Self {
        InternalDataVariant::EnumVariant(enum_variant)
    }
}

impl<'data> InternalDataVariant<'data> {
    /// Returns the internal crate dependencies of the variant.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        match self {
            InternalDataVariant::StructVariant(s) => s.internal_dependencies(),
            InternalDataVariant::EnumVariant(e) => e.internal_dependencies(),
        }
    }

    /// Returns whether the underlying variant supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_variant` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_variant: &TraitVariantRef<'data>) -> bool {
        match self {
            InternalDataVariant::StructVariant(s) => s.supports_trait(trait_variant),
            InternalDataVariant::EnumVariant(e) => e.supports_trait(trait_variant),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Enum representing a variant of internal data, which may be defined within
/// the workspace or come from an external crate.
pub enum DataVariantRef<'data> {
    /// Variant representing internal data defined within the workspace.
    Internal(InternalDataRef<'data>),
    /// Variant representing data defined within an external crate.
    External(ExternalTypeRef<'data>),
    /// A reference to a data variant ref.
    Reference(Option<Lifetime>, Box<DataVariantRef<'data>>),
    /// A mutable reference to a data variant ref.
    MutableReference(Option<Lifetime>, Box<DataVariantRef<'data>>),
    /// A generic type parameter.
    Generic(Ident),
}

impl Debug for DataVariantRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataVariantRef::Internal(internal) => write!(f, "Internal({internal:?})"),
            DataVariantRef::External(external) => write!(f, "External({external:?})"),
            DataVariantRef::Reference(_, inner) => write!(f, "Reference({inner:?})"),
            DataVariantRef::MutableReference(_, inner) => {
                write!(f, "MutableReference({inner:?})")
            }
            DataVariantRef::Generic(ident) => write!(f, "Generic({ident})"),
        }
    }
}

impl<'data> From<InternalDataRef<'data>> for DataVariantRef<'data> {
    fn from(internal: InternalDataRef<'data>) -> Self {
        DataVariantRef::Internal(internal)
    }
}

impl<'data> From<ExternalTypeRef<'data>> for DataVariantRef<'data> {
    fn from(external: ExternalTypeRef<'data>) -> Self {
        DataVariantRef::External(external)
    }
}

impl<'data> InternalDependencies<'data> for DataVariantRef<'data> {
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate<'data>> {
        match self {
            DataVariantRef::Internal(internal) => internal.internal_dependencies(),
            DataVariantRef::External(_) => vec![],
            DataVariantRef::Reference(_, inner) => inner.internal_dependencies(),
            DataVariantRef::MutableReference(_, inner) => inner.internal_dependencies(),
            DataVariantRef::Generic(_) => vec![],
        }
    }
}

impl<'data> crate::traits::ExternalDependencies<'data> for DataVariantRef<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        match self {
            DataVariantRef::Internal(_) => vec![],
            DataVariantRef::External(external) => vec![external.external_crate()],
            DataVariantRef::Reference(_, inner) => inner.external_dependencies(),
            DataVariantRef::MutableReference(_, inner) => inner.external_dependencies(),
            DataVariantRef::Generic(_) => vec![],
        }
    }
}

impl<'data> DataVariantRef<'data> {
    /// Creates a new generic `DataVariantRef`.
    pub fn generic(ident: Ident) -> DataVariantRef<'data> {
        DataVariantRef::Generic(ident)
    }

    /// Returns whether the underlying variant supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        match self {
            DataVariantRef::Internal(internal) => {
                internal.data().variant().supports_trait(trait_ref)
            }
            DataVariantRef::External(external) => external.supports_trait(trait_ref),
            DataVariantRef::Reference(_lifetime, inner) => inner.supports_trait(trait_ref),
            DataVariantRef::MutableReference(_lifetime, inner) => inner.supports_trait(trait_ref),
            DataVariantRef::Generic(_) => false,
        }
    }

    /// Returns whether the variant is a mutable reference.
    pub fn is_mutable_reference(&self) -> bool {
        matches!(self, DataVariantRef::MutableReference(_, _))
    }

    /// Returns whether the variant is a reference.
    pub fn is_reference(&self) -> bool {
        matches!(self, DataVariantRef::Reference(_, _))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to internal data and its crate.
pub struct InternalDataRef<'data> {
    data: Rc<InternalData<'data>>,
    internal_crate: Rc<InternalCrate<'data>>,
}

impl<'data> InternalDataRef<'data> {
    /// Creates a new `InternalDataRef`.
    ///
    /// # Arguments
    ///
    /// * `internal_crate` - A reference to the internal crate.
    /// * `data` - A reference to the internal data.
    pub fn new(
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

impl Display for InternalDataRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let internal_crate_name = self.internal_crate.name();
        let internal_data_name = self.data.name();
        write!(f, "{internal_crate_name}::prelude::{internal_data_name}")
    }
}

impl ToTokens for InternalDataRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let internal_crate_ident = self.internal_crate.ident();
        let internal_data_ident = self.data.ident();
        tokens.extend(quote! {#internal_crate_ident::prelude::#internal_data_ident});
    }
}

impl ToTokens for DataVariantRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            DataVariantRef::Internal(internal) => {
                internal.to_tokens(tokens);
            }
            DataVariantRef::External(external) => {
                external.rust_type().to_tokens(tokens);
            }
            DataVariantRef::Reference(lifetime, inner) => {
                tokens.extend(quote! {& #lifetime });
                inner.to_tokens(tokens);
            }
            DataVariantRef::MutableReference(lifetime, inner) => {
                tokens.extend(quote! {&mut #lifetime });
                inner.to_tokens(tokens);
            }
            DataVariantRef::Generic(ident) => {
                ident.to_tokens(tokens);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to internal module and its crate.
pub struct InternalModuleRef<'data> {
    module: Rc<InternalModule<'data>>,
    internal_crate: Rc<InternalCrate<'data>>,
}

impl ToTokens for InternalModuleRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = self
            .internal_crate
            .module_path(self.module.as_ref())
            .expect("Failed to get module path for internal module ref");
        tokens.extend(quote! {#path});
    }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct defining a data model.
pub struct InternalData<'data> {
    /// Publicness of the data.
    publicness: Publicness,
    /// Name of the data.
    name: String,
    /// Documentation of the data.
    documentation: Option<String>,
    /// The variant of the data (struct or enum).
    variant: InternalDataVariant<'data>,
    /// The traits implemented for the data.
    traits: Vec<TraitVariantRef<'data>>,
    /// The derives applies to the data.
    derives: Vec<Derive<'data>>,
    /// The decorators applied to the data which are not derives.
    decorators: Vec<Decorator<'data>>,
}

impl<'data> InternalData<'data> {
    /// Initializes a new `InternalDataBuilder`.
    pub fn new() -> InternalDataBuilder<'data> {
        InternalDataBuilder::default()
    }

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
        let documentation = match &self.documentation {
            Some(doc) => {
                quote::quote! {
                    #[doc = #doc]
                }
            }
            None => quote::quote! {},
        };
        let derives = &self.derives;
        let decorators = &self.decorators;
        let token = quote::quote! {
            #(#derives)*
            #(#decorators)*
            #documentation
            #publicness #variant
        };
        tokens.extend(token);
    }
}

impl<'data> ExternalDependencies<'data> for InternalData<'data> {
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
        let mut crates = self
            .traits
            .iter()
            .filter_map(|t| {
                if let TraitVariantRef::External(ext_trait_ref) = t {
                    Some(ext_trait_ref.external_crate())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for derive in &self.derives {
            crates.extend(derive.external_dependencies());
        }

        for decorator in &self.decorators {
            crates.extend(decorator.external_dependencies());
        }

        crates.sort_unstable();
        crates.dedup();
        crates
    }
}

impl<'data> InternalDependencies<'data> for InternalData<'data> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        let mut crates = self
            .traits
            .iter()
            .filter_map(|t| {
                if let TraitVariantRef::Internal(_, krate) = t { Some(*krate) } else { None }
            })
            .collect::<Vec<_>>();

        for derive in &self.derives {
            crates.extend(derive.internal_dependencies());
        }

        for decorator in &self.decorators {
            crates.extend(decorator.internal_dependencies());
        }

        crates.sort_unstable();
        crates.dedup();
        crates
    }
}
