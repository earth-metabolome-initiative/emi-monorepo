//! Procedural macro to implement the traits relative to the validation of String using custom Validators.
//!
//! The macro takes as parameter the error message to be used in case of validation failure.
//!
//! # Example
//! The usage example of the custom_validator macro is as follows:
//!
//! ```rust
//! use web_common::custom_validator;
//!
//! #[custom_validator("No trailing spaces allowed.")]
//! pub fn no_trailing_spaces<S>(v: &S) -> Result<(), ValidationError> {
//!     if v.as_ref().ends_with(' ') {
//!         return Err(ValidationError::new("no_trailing_spaces"));
//!     }
//!     Ok(())
//! }
//! ```
//!
//! This decorated function will generate the struct #struct_name and
//! implement several traits for it, including the TryFrom trait.

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn custom_validator(args: TokenStream, mut input: TokenStream) -> TokenStream {
    let mut args_iter = args.clone().into_iter();

    let error_message: String = match args_iter.next() {
        Some(proc_macro::TokenTree::Literal(literal)) => {
            literal.to_string().trim_matches('"').to_string()
        }
        _ => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "Error message must be a non-empty string enclosed in double quotes.",
            )
            .to_compile_error()
            .into();
        }
    };

    // We check that there are no more arguments, as we only expect one.
    if args_iter.next().is_some() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Only one argument is expected.",
        )
        .to_compile_error()
        .into();
    }

    let input_cloned = input.clone();
    let parsed_input = parse_macro_input!(input_cloned as syn::ItemFn);

    let function_name = parsed_input.sig.ident.to_string();

    // We generate the name of the struct by splitting the name of the decorated
    // function and capitalizing the first letter of each word.
    let struct_name = function_name
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<String>();

    let struct_name = syn::Ident::new(&struct_name, parsed_input.sig.ident.span());

    let expanded = quote::quote! {
        #[automatically_derived]
        #[repr(transparent)]
        #[derive(Debug, validator::Validate, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, Clone, Default, serde::Deserialize)]
        pub struct #struct_name<S=crate::custom_validators::ValidatableString>
        where
            S: AsRef<str> + serde::Serialize + validator::Validate,
        {
            #[validate]
            #[validate(custom(function=#function_name, message=#error_message))]
            value: S,
        }

        impl<S> #struct_name<S>
        where
            S: AsRef<str> + serde::Serialize + validator::Validate,
        {
            pub fn is_empty(&self) -> bool {
                self.value.as_ref().is_empty()
            }
        }

        impl<S> TryFrom<String> for #struct_name<S>
        where S: crate::custom_validators::validation_errors::TryFromString + AsRef<str> + serde::Serialize + validator::Validate,
        {
            type Error = crate::api::ApiError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                use validator::Validate;
                use crate::custom_validators::validation_errors::ValidationErrorToString;
                let maybe_self = Self { value: S::try_from_string(value)? };
                if let Err(e) = maybe_self.validate() {
                    return Err(e.convert_to_string().into());
                }
                Ok(maybe_self)
            }
        }

        impl<S> crate::custom_validators::validation_errors::TryFromString for #struct_name<S>
        where S: AsRef<str> + serde::Serialize + validator::Validate + crate::custom_validators::validation_errors::TryFromString
        {
            fn try_from_string(value: String) -> Result<Self, crate::api::ApiError> {
                use validator::Validate;
                use crate::custom_validators::validation_errors::ValidationErrorToString;
                let maybe_self = Self { value: S::try_from_string(value)? };
                if let Err(e) = maybe_self.validate() {
                    return Err(e.convert_to_string().into());
                }
                Ok(maybe_self)
            }
        }

        impl<S> AsRef<str> for #struct_name<S>
        where
            S: AsRef<str> + serde::Serialize + validator::Validate,
        {
            fn as_ref(&self) -> &str {
                self.value.as_ref()
            }
        }

        impl<S> validator::HasLen for &#struct_name<S>
        where
            S: AsRef<str> + serde::Serialize + validator::Validate,
        {
            fn length(&self) -> u64 {
                self.value.as_ref().len() as u64
            }
        }

        impl<S> std::fmt::Display for #struct_name<S>
        where
            S: AsRef<str> + serde::Serialize + validator::Validate,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value.as_ref())
            }
        }
    };

    input.extend(TokenStream::from(expanded));

    input
}

#[proc_macro_attribute]
pub fn image_validator(args: TokenStream, mut input: TokenStream) -> TokenStream {
    let mut args_iter = args.clone().into_iter();

    let error_message: String = match args_iter.next() {
        Some(proc_macro::TokenTree::Literal(literal)) => {
            literal.to_string().trim_matches('"').to_string()
        }
        _ => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "Error message must be a non-empty string enclosed in double quotes.",
            )
            .to_compile_error()
            .into();
        }
    };

    // We check that there are no more arguments, as we only expect one.
    if args_iter.next().is_some() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Only one argument is expected.",
        )
        .to_compile_error()
        .into();
    }

    let input_cloned = input.clone();
    let parsed_input = parse_macro_input!(input_cloned as syn::ItemFn);

    let function_name = parsed_input.sig.ident.to_string();

    // We generate the name of the struct by splitting the name of the decorated
    // function and capitalizing the first letter of each word.
    let struct_name = function_name
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<String>();

    let struct_name = syn::Ident::new(&struct_name, parsed_input.sig.ident.span());

    let expanded = quote::quote! {

        #[automatically_derived]
        #[repr(transparent)]
        #[derive(Debug, validator::Validate, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, Clone, Default, serde::Deserialize)]
        pub struct #struct_name<S=crate::custom_validators::images::image::Image>
        where
            S: AsRef<Image> + serde::Serialize + validator::Validate,
        {
            #[validate]
            #[validate(custom(function=#function_name, message=#error_message))]
            value: S,
        }

        impl<S> TryFrom<Image> for #struct_name<S>
        where S: crate::custom_validators::image::TryFromImage + AsRef<Image> + serde::Serialize + validator::Validate,        {
            type Error = crate::api::ApiError;

            fn try_from(value: Image) -> Result<Self, Self::Error> {
                use validator::Validate;
                use crate::custom_validators::validation_errors::ValidationErrorToString;
                let maybe_self = Self { value: S::try_from_image(value)?};
                if let Err(e) = maybe_self.validate() {
                    return Err(e.convert_to_string().into());
                }
                Ok(maybe_self)
            }
        }

        impl<S> crate::custom_validators::image::TryFromImage for #struct_name<S>
        where S: crate::custom_validators::image::TryFromImage + AsRef<Image> + serde::Serialize + validator::Validate,        {
            fn try_from_image(image: Image) -> Result<Self, crate::api::ApiError> {
                use validator::Validate;
                use crate::custom_validators::validation_errors::ValidationErrorToString;
                let maybe_self = Self { value: S::try_from_image(image)?};
                if let Err(e) = maybe_self.validate() {
                    return Err(e.convert_to_string().into());
                }
                Ok(maybe_self)
            }
        }

        #[cfg(feature = "frontend")]
        impl<S> crate::api::form_traits::TryFromCallback<web_sys::File> for #struct_name<S>
            where
            S: crate::api::form_traits::TryFromCallback<web_sys::File> + AsRef<Image> + serde::Serialize + validator::Validate,        {
            fn try_from_callback<C>(file: web_sys::File, callback: C) -> Result<(), crate::api::ApiError>
            where
                C: FnOnce(Result<Self, crate::api::ApiError>) + 'static,
            {
                use validator::Validate;
                use crate::custom_validators::validation_errors::ValidationErrorToString;
                S::try_from_callback(file, move |value| {
                    match value {
                        Err(e) => callback(Err(e)),
                        Ok(value) => {
                            let maybe_self = Self { value };
                            match maybe_self.validate() {
                                Ok(()) => callback(Ok(maybe_self)),
                                Err(e) => callback(Err(e.convert_to_string().into())),
                            };
                        }
                    };
                })
            }
        }

        impl<S> AsRef<Image> for #struct_name<S>
        where
            S: AsRef<Image> + serde::Serialize + validator::Validate,
        {
            fn as_ref(&self) -> &Image {
                self.value.as_ref()
            }
        }

        impl<S> From<#struct_name<S>> for Image
        where
            S: AsRef<Image> + serde::Serialize + validator::Validate,
            S: Into<Image>,
        {
            fn from(value: #struct_name<S>) -> Image {
                value.value.into()
            }
        }
    };

    input.extend(TokenStream::from(expanded));

    input
}
