//! Submodule providing a struct which defines a data model.

mod builder;

use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub use builder::InternalDataBuilder;
use common_traits::prelude::Builder;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Lifetime};

use crate::{
    structs::{
        Decorator, Derive, Documentation, ExternalCrate, InternalCrate, InternalEnum,
        InternalModule, InternalStruct, InternalToken, Publicness, external_crate::ExternalTypeRef,
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

impl<'data> ExternalDependencies<'data> for InternalDataVariant<'data> {
    fn external_dependencies(&self) -> Vec<&crate::structs::ExternalCrate<'data>> {
        match self {
            InternalDataVariant::StructVariant(s) => s.external_dependencies(),
            InternalDataVariant::EnumVariant(e) => e.external_dependencies(),
        }
    }
}

impl<'data> InternalDependencies<'data> for InternalDataVariant<'data> {
    fn internal_dependencies(&self) -> Vec<&crate::structs::InternalCrate<'data>> {
        match self {
            InternalDataVariant::StructVariant(s) => s.internal_dependencies(),
            InternalDataVariant::EnumVariant(e) => e.internal_dependencies(),
        }
    }
}

impl<'data> InternalDataVariant<'data> {
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
    /// A result type.
    Result(Box<DataVariantRef<'data>>, Box<DataVariantRef<'data>>),
    /// A option type.
    Option(Box<DataVariantRef<'data>>),
    /// A self type, of which sometimes it is known what it is.
    SelfType(Option<Box<DataVariantRef<'data>>>),
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
            DataVariantRef::Result(left, right) => {
                write!(f, "Result({left:?}, {right:?})")
            }
            DataVariantRef::Option(inner) => {
                write!(f, "Option({inner:?})")
            }
            DataVariantRef::SelfType(inner) => {
                write!(f, "Self<{inner:?}>")
            }
        }
    }
}

impl<'data> From<InternalDataRef<'data>> for DataVariantRef<'data> {
    fn from(internal: InternalDataRef<'data>) -> Self {
        DataVariantRef::Internal(internal)
    }
}

impl<'data> From<InternalData<'data>> for DataVariantRef<'data> {
    fn from(internal: InternalData<'data>) -> Self {
        DataVariantRef::Internal(internal.into())
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
            DataVariantRef::Result(left, right) => {
                let mut crates = left.internal_dependencies();
                crates.extend(right.internal_dependencies());
                crates
            }
            DataVariantRef::Option(inner) => inner.internal_dependencies(),
            DataVariantRef::SelfType(inner) => inner.internal_dependencies(),
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
            DataVariantRef::Result(left, right) => {
                let mut crates = left.external_dependencies();
                crates.extend(right.external_dependencies());
                crates
            }
            DataVariantRef::Option(inner) => inner.external_dependencies(),
            DataVariantRef::SelfType(inner) => inner.external_dependencies(),
        }
    }
}

impl<'data> DataVariantRef<'data> {
    /// Creates a new generic `DataVariantRef`.
    pub fn generic(ident: Ident) -> DataVariantRef<'data> {
        DataVariantRef::Generic(ident)
    }

    /// Returns a reference variant of the data variant.
    pub fn reference(&self, lifetime: Option<Lifetime>) -> DataVariantRef<'data> {
        DataVariantRef::Reference(lifetime, Box::new(self.clone()))
    }

    /// Returns a mutable reference variant of the data variant.
    pub fn mutable_reference(&self, lifetime: Option<Lifetime>) -> DataVariantRef<'data> {
        DataVariantRef::MutableReference(lifetime, Box::new(self.clone()))
    }

    /// Returns an iterator over the generics without a default used in the
    /// data variant.
    pub fn generics_without_defaults(&self) -> Vec<&Ident> {
        match self {
            DataVariantRef::Internal(internal) => {
                internal.data().generics_without_defaults().collect()
            }
            DataVariantRef::External(_) => vec![],
            DataVariantRef::Reference(_, inner) => inner.generics_without_defaults(),
            DataVariantRef::MutableReference(_, inner) => inner.generics_without_defaults(),
            DataVariantRef::Generic(ident) => vec![ident],
            DataVariantRef::Result(left, right) => {
                let mut generics = left.generics_without_defaults();
                generics.extend(right.generics_without_defaults());
                generics.sort_unstable();
                generics.dedup();
                generics
            }
            DataVariantRef::Option(inner) => inner.generics_without_defaults(),
            DataVariantRef::SelfType(inner) => {
                if let Some(inner) = inner {
                    inner.generics_without_defaults()
                } else {
                    vec![]
                }
            }
        }
    }

    /// Formats the variant including the generics, if any, with defaults.
    pub fn format_with_generics(&self) -> TokenStream {
        match self {
            Self::External(external) => quote! {#external},
            Self::Internal(internal) => internal.format_with_generics(),
            Self::Generic(generic) => quote! {#generic},
            Self::Reference(_, inner) => {
                let inner_tokens = inner.format_with_generics();
                quote! { & #inner_tokens }
            }
            Self::MutableReference(_, inner) => {
                let inner_tokens = inner.format_with_generics();
                quote! { &mut #inner_tokens }
            }
            Self::Result(left, right) => {
                let left_tokens = left.format_with_generics();
                let right_tokens = right.format_with_generics();
                quote! { Result<#left_tokens, #right_tokens> }
            }
            Self::Option(inner) => {
                let inner_tokens = inner.format_with_generics();
                quote! { Option<#inner_tokens> }
            }
            Self::SelfType(_) => {
                quote! { Self }
            }
        }
    }

    /// Returns the dereferenced variant if it is a reference or mutable
    /// reference, otherwise returns itself.
    pub fn dereference(&self) -> &DataVariantRef<'data> {
        match self {
            DataVariantRef::Reference(_, inner) => inner.as_ref(),
            DataVariantRef::MutableReference(_, inner) => inner.as_ref(),
            _ => self,
        }
    }

    /// Returns whether the underlying variant supports the given trait.
    ///
    /// # Arguments
    ///
    /// * `trait_ref` - The trait variant to check support for.
    pub fn supports_trait(&self, trait_ref: &TraitVariantRef<'data>) -> bool {
        match self {
            Self::Internal(internal) => internal.data().variant().supports_trait(trait_ref),
            Self::External(external) => external.supports_trait(trait_ref),
            Self::Reference(_lifetime, inner) => inner.supports_trait(trait_ref),
            Self::MutableReference(_lifetime, inner) => inner.supports_trait(trait_ref),
            Self::Generic(_) => false,
            Self::Result(_, _) => false,
            Self::Option(inner) => inner.supports_trait(trait_ref),
            Self::SelfType(inner) => {
                match inner {
                    Some(inner_variant) => inner_variant.supports_trait(trait_ref),
                    None => false,
                }
            }
        }
    }

    /// Returns whether the variant is a mutable reference.
    pub fn is_mutable_reference(&self) -> bool {
        matches!(self, Self::MutableReference(_, _))
    }

    /// Returns whether the variant is a reference.
    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_, _))
    }

    /// Returns whether it is a `Result` variant.
    pub fn is_result(&self) -> bool {
        matches!(self, Self::Result(_, _))
    }

    /// Returns whether it is an `Option` variant.
    pub fn is_option(&self) -> bool {
        matches!(self, Self::Option(_))
    }

    /// Returns whether it is a `Self` type variant.
    pub fn is_self_type(&self) -> bool {
        matches!(self, Self::SelfType(_))
            || matches!(self, Self::Reference(_, inner) if inner.is_self_type())
            || matches!(self, Self::MutableReference(_, inner) if inner.is_self_type())
    }

    /// Creates a new `Result` variant from the given left and right variants.
    ///
    /// # Arguments
    /// * `left` - The left variant of the `Result`.
    /// * `right` - The right variant of the `Result`.
    pub fn result<Ok, Err>(ok_variant: Ok, err_variant: Err) -> DataVariantRef<'data>
    where
        Ok: Into<DataVariantRef<'data>>,
        Err: Into<DataVariantRef<'data>>,
    {
        DataVariantRef::Result(Box::new(ok_variant.into()), Box::new(err_variant.into()))
    }

    /// Creates a new `Option` variant from the given inner variant.
    ///
    /// # Arguments
    ///
    /// * `inner` - The inner variant of the `Option`.
    pub fn optional(&self) -> DataVariantRef<'data> {
        DataVariantRef::Option(Box::new(self.clone()))
    }

    /// Creates a new `Self` type variant with the optional inner variant.
    ///
    /// # Arguments
    ///
    /// * `inner` - The optional inner variant of the `Self` type.
    pub fn self_type(inner: Option<DataVariantRef<'data>>) -> DataVariantRef<'data> {
        DataVariantRef::SelfType(inner.map(Box::new))
    }

    /// Creates a new Result with `diesel::result::Error` as the error type.
    ///
    /// # Arguments
    /// * `ok_variant` - The ok variant of the Result.
    pub fn diesel_result(ok_variant: DataVariantRef<'data>) -> DataVariantRef<'data> {
        let diesel = ExternalCrate::diesel();
        let err_variant = diesel
            .external_type(&syn::parse_quote!(diesel::result::Error))
            .expect("Failed to get diesel::result::Error external type");
        Self::result(ok_variant, err_variant)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Struct representing a reference to internal data and its crate.
pub struct InternalDataRef<'data> {
    data: Rc<InternalData<'data>>,
    /// The crate in which the internal data is defined.
    ///
    /// This is optional to allow for cases where the crate
    /// itself has not yet been defined (e.g., for the current crate).
    internal_crate: Option<Rc<InternalCrate<'data>>>,
}

impl<'data> From<InternalDataRef<'data>> for InternalToken<'data> {
    fn from(value: InternalDataRef<'data>) -> Self {
        InternalToken::new()
            .public()
            .stream(value.to_token_stream())
            .data(value.into())
            .unwrap()
            .build()
            .unwrap()
    }
}

impl<'data> From<Rc<InternalData<'data>>> for InternalDataRef<'data> {
    fn from(data: Rc<InternalData<'data>>) -> Self {
        InternalDataRef { data, internal_crate: None }
    }
}

impl<'data> From<InternalData<'data>> for InternalDataRef<'data> {
    fn from(data: InternalData<'data>) -> Self {
        InternalDataRef { data: Rc::new(data), internal_crate: None }
    }
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
        InternalDataRef { data: data.clone(), internal_crate: Some(internal_crate.clone()) }
    }

    /// Returns the internal data.
    pub fn data(&self) -> &InternalData<'data> {
        self.data.as_ref()
    }

    /// Returns the internal crate.
    pub fn internal_crate(&self) -> Option<&InternalCrate<'data>> {
        self.internal_crate.as_deref()
    }

    /// Returns the internal crate Rc reference.
    pub fn crate_ref(&self) -> Option<&Rc<InternalCrate<'data>>> {
        self.internal_crate.as_ref()
    }

    /// Returns the name of the internal data.
    pub fn name(&self) -> &str {
        self.data.name()
    }

    /// Returns the ident of the internal crate.
    pub fn crate_ident(&self) -> Option<Ident> {
        self.internal_crate.as_ref().map(|krate| krate.ident())
    }

    /// Returns the internal crate dependencies of the variant.
    pub fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        self.internal_crate().into_iter().collect()
    }

    /// Returns the markdown formatted documentation path of the internal data
    /// type.
    pub fn documentation_path(&self) -> String {
        format!("[`{}`]({self})", self.name())
    }

    /// Formats the variant including the generics, if any, with defaults.
    pub fn format_with_generics(&self) -> TokenStream {
        let generics = self.data.generics_with_defaults();
        quote::quote! { #self #generics }
    }
}

impl Display for InternalDataRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let internal_crate_name = if let Some(internal_crate) = &self.internal_crate {
            internal_crate.name()
        } else {
            "crate"
        };
        let internal_data_name = self.data.name();
        write!(f, "{internal_crate_name}::prelude::{internal_data_name}")
    }
}

impl ToTokens for InternalDataRef<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let internal_crate_ident = if let Some(internal_crate) = &self.internal_crate {
            let ident = internal_crate.ident();
            quote::quote! {#ident}
        } else {
            quote::quote! {crate}
        };
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
            DataVariantRef::Result(left, right) => {
                tokens.extend(quote! {Result<#left, #right>});
            }
            DataVariantRef::Option(inner) => {
                tokens.extend(quote! {Option<#inner>});
            }
            DataVariantRef::SelfType(_) => {
                tokens.extend(quote! {Self});
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
    documentation: Documentation<'data>,
    /// The variant of the data (struct or enum).
    variant: InternalDataVariant<'data>,
    /// The traits implemented for the data.
    traits: Vec<InternalToken<'data>>,
    /// The derives applies to the data.
    derives: Vec<Derive<'data>>,
    /// The decorators applied to the data which are not derives.
    decorators: Vec<Decorator<'data>>,
    /// The generics used in the data.
    generics: Vec<Ident>,
    /// Defaults for generic type parameters.
    generic_defaults: Vec<Option<DataVariantRef<'data>>>,
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

    /// Returns the formatted generics of the data.
    pub fn generics(&self) -> Option<TokenStream> {
        if self.generics.is_empty() {
            None
        } else {
            let generics = &self.generics;
            Some(quote::quote! { <#(#generics),*> })
        }
    }

    /// Returns the formatted generics, with defaults in place of the generic
    /// where they exist.
    pub fn generics_with_defaults(&self) -> Option<TokenStream> {
        if self.generics.is_empty() {
            None
        } else {
            let generics_with_defaults =
                self.generics.iter().zip(self.generic_defaults.iter()).map(|(ident, default)| {
                    if let Some(default) = default {
                        quote::quote! { #default }
                    } else {
                        quote::quote! { #ident }
                    }
                });
            Some(quote::quote! { <#(#generics_with_defaults),*> })
        }
    }

    /// Returns an iterator over the generics for which no default is defined.
    pub fn generics_without_defaults(&self) -> impl Iterator<Item = &Ident> {
        self.generics
            .iter()
            .zip(self.generic_defaults.iter())
            .filter_map(|(ident, default)| if default.is_none() { Some(ident) } else { None })
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
        let maybe_generics = self.generics();

        let variant = match &self.variant {
            InternalDataVariant::StructVariant(s) => {
                quote::quote! {
                    struct #ident #maybe_generics #s
                }
            }
            InternalDataVariant::EnumVariant(e) => {
                quote::quote! {
                    enum #ident #maybe_generics #e
                }
            }
        };
        let documentation = &self.documentation;
        let derives = &self.derives;
        let decorators = &self.decorators;
        let traits = &self.traits;
        let token = quote::quote! {
            #(#derives)*
            #(#decorators)*
            #documentation
            #publicness #variant
            #(#traits)*
        };
        tokens.extend(token);
    }
}

impl<'data> ExternalDependencies<'data> for InternalData<'data> {
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
        let mut crates = self
            .traits
            .iter()
            .flat_map(ExternalDependencies::external_dependencies)
            .collect::<Vec<_>>();

        for derive in &self.derives {
            crates.extend(derive.external_dependencies());
        }

        for decorator in &self.decorators {
            crates.extend(decorator.external_dependencies());
        }

        crates.extend(self.variant.external_dependencies());

        crates.extend(self.documentation.external_dependencies());

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
            .flat_map(InternalDependencies::internal_dependencies)
            .collect::<Vec<_>>();

        for derive in &self.derives {
            crates.extend(derive.internal_dependencies());
        }

        for decorator in &self.decorators {
            crates.extend(decorator.internal_dependencies());
        }

        crates.extend(self.variant.internal_dependencies());

        crates.extend(self.documentation.internal_dependencies());

        crates.sort_unstable();
        crates.dedup();
        crates
    }
}
