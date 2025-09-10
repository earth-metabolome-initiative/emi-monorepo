//! Crate providing common validation errors.

mod from;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
/// Enumeration of errors that can occur during validation.
pub enum Error<FieldName = ()> {
    /// Single field errors.
    SingleField(SingleFieldError<FieldName>),
    /// Double field errors.
    DoubleField(DoubleFieldError<FieldName>),
}

impl<FieldName> Error<FieldName> {
    /// Converts the error into the provided new attribute type.
    pub fn into_field_name<F, NewFieldName>(self, convert: F) -> Error<NewFieldName>
    where
        F: Fn(FieldName) -> NewFieldName,
    {
        match self {
            Error::SingleField(error) => Error::SingleField(error.into_field_name(convert)),
            Error::DoubleField(error) => Error::DoubleField(error.into_field_name(convert)),
        }
    }
}

impl<FieldName: core::fmt::Display + core::fmt::Debug> core::error::Error for Error<FieldName> {}

impl<FieldName: core::fmt::Display> core::fmt::Display for Error<FieldName> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::SingleField(error) => {
                <SingleFieldError<FieldName> as core::fmt::Display>::fmt(error, f)
            }
            Error::DoubleField(error) => {
                <DoubleFieldError<FieldName> as core::fmt::Display>::fmt(error, f)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
/// Enumeration of errors that can occur during validation of a single field.
pub enum SingleFieldError<FieldName = ()> {
    /// The provided text is empty.
    EmptyText(FieldName),
    /// The provided text is padded with spaces.
    PaddedText(FieldName),
    /// The provided text contains control characters.
    ControlCharacters(FieldName),
    /// The provided text contains consecutive whitespace characters.
    ConsecutiveWhitespace(FieldName),
    /// The provided mail address is invalid.
    InvalidMail(FieldName),
    /// The provided text is not a valid font awesome class.
    InvalidFontAwesomeClass(FieldName, String),
    /// The numeric value is not strictly positive (0.0, ...]
    UnexpectedNegativeOrZeroValue(FieldName),
    /// The numeric value is not positive [0.0, ...]
    UnexpectedNegativeValue(FieldName),
    /// The float is not strictly greater than the expected amount.
    MustBeSmallerThan(FieldName, f64),
    /// The float is not strictly smaller than the expected amount.
    MustBeGreaterThan(FieldName, f64),
    /// The provided text is not a valid cas code.
    InvalidCasCode(FieldName, cas_codes::errors::Error),
    /// The provided text is not a valid Molecular Formula.
    InvalidMolecularFormula(FieldName, molecular_formulas::errors::Error),
    /// The provided text is not a valid media type.
    InvalidMediaType(FieldName, media_types::errors::Error),
}

impl SingleFieldError {
    /// Renames the field name.
    pub fn rename_field<FieldName>(self, field_name: FieldName) -> SingleFieldError<FieldName> {
        match self {
            SingleFieldError::EmptyText(()) => SingleFieldError::EmptyText(field_name),
            SingleFieldError::PaddedText(()) => SingleFieldError::PaddedText(field_name),
            SingleFieldError::ConsecutiveWhitespace(()) => {
                SingleFieldError::ConsecutiveWhitespace(field_name)
            }
            SingleFieldError::ControlCharacters(()) => {
                SingleFieldError::ControlCharacters(field_name)
            }
            SingleFieldError::InvalidMail(()) => SingleFieldError::InvalidMail(field_name),
            SingleFieldError::InvalidFontAwesomeClass((), icon) => {
                SingleFieldError::InvalidFontAwesomeClass(field_name, icon)
            }
            SingleFieldError::UnexpectedNegativeOrZeroValue(()) => {
                SingleFieldError::UnexpectedNegativeOrZeroValue(field_name)
            }
            SingleFieldError::UnexpectedNegativeValue(()) => {
                SingleFieldError::UnexpectedNegativeValue(field_name)
            }
            SingleFieldError::MustBeSmallerThan((), expected_value) => {
                SingleFieldError::MustBeSmallerThan(field_name, expected_value)
            }
            SingleFieldError::MustBeGreaterThan((), expected_value) => {
                SingleFieldError::MustBeGreaterThan(field_name, expected_value)
            }
            SingleFieldError::InvalidCasCode((), error) => {
                SingleFieldError::InvalidCasCode(field_name, error)
            }
            SingleFieldError::InvalidMolecularFormula((), error) => {
                SingleFieldError::InvalidMolecularFormula(field_name, error)
            }
            SingleFieldError::InvalidMediaType((), error) => {
                SingleFieldError::InvalidMediaType(field_name, error)
            }
        }
    }

    /// Turns the error into its double field equivalent and sets the field
    /// names.
    ///
    /// # Panics
    ///
    /// * If the variant cannot be converted into a double field error.
    pub fn rename_fields<FieldName>(
        self,
        left: FieldName,
        right: FieldName,
    ) -> DoubleFieldError<FieldName> {
        match self {
            SingleFieldError::MustBeSmallerThan((), _) => {
                DoubleFieldError::MustBeSmallerThan(left, right)
            }
            SingleFieldError::MustBeGreaterThan((), _) => {
                DoubleFieldError::MustBeGreaterThan(left, right)
            }
            SingleFieldError::EmptyText(())
            | SingleFieldError::PaddedText(())
            | SingleFieldError::ConsecutiveWhitespace(())
            | SingleFieldError::ControlCharacters(())
            | SingleFieldError::InvalidFontAwesomeClass((), _)
            | SingleFieldError::InvalidMail(())
            | SingleFieldError::InvalidCasCode((), _)
            | SingleFieldError::InvalidMolecularFormula((), _)
            | SingleFieldError::InvalidMediaType((), _)
            | SingleFieldError::UnexpectedNegativeValue(())
            | SingleFieldError::UnexpectedNegativeOrZeroValue(()) => {
                unimplemented!("Cannot convert the variant error into a double field error.")
            }
        }
    }
}

impl<FieldName> SingleFieldError<FieldName> {
    /// Converts the error into the provided new attribute type.
    pub fn into_field_name<F, NewFieldName>(self, convert: F) -> SingleFieldError<NewFieldName>
    where
        F: Fn(FieldName) -> NewFieldName,
    {
        match self {
            SingleFieldError::EmptyText(field_name) => {
                SingleFieldError::EmptyText(convert(field_name))
            }
            SingleFieldError::PaddedText(field_name) => {
                SingleFieldError::PaddedText(convert(field_name))
            }
            SingleFieldError::ConsecutiveWhitespace(field_name) => {
                SingleFieldError::ConsecutiveWhitespace(convert(field_name))
            }
            SingleFieldError::ControlCharacters(field_name) => {
                SingleFieldError::ControlCharacters(convert(field_name))
            }
            SingleFieldError::InvalidMail(field_name) => {
                SingleFieldError::InvalidMail(convert(field_name))
            }
            SingleFieldError::InvalidFontAwesomeClass(field_name, icon) => {
                SingleFieldError::InvalidFontAwesomeClass(convert(field_name), icon)
            }
            SingleFieldError::UnexpectedNegativeOrZeroValue(field_name) => {
                SingleFieldError::UnexpectedNegativeOrZeroValue(convert(field_name))
            }
            SingleFieldError::UnexpectedNegativeValue(field_name) => {
                SingleFieldError::UnexpectedNegativeValue(convert(field_name))
            }
            SingleFieldError::MustBeSmallerThan(field_name, expected_value) => {
                SingleFieldError::MustBeSmallerThan(convert(field_name), expected_value)
            }
            SingleFieldError::MustBeGreaterThan(field_name, expected_value) => {
                SingleFieldError::MustBeGreaterThan(convert(field_name), expected_value)
            }
            SingleFieldError::InvalidCasCode(field_name, error) => {
                SingleFieldError::InvalidCasCode(convert(field_name), error)
            }
            SingleFieldError::InvalidMolecularFormula(field_name, error) => {
                SingleFieldError::InvalidMolecularFormula(convert(field_name), error)
            }
            SingleFieldError::InvalidMediaType(field_name, error) => {
                SingleFieldError::InvalidMediaType(convert(field_name), error)
            }
        }
    }
}

impl<A> From<SingleFieldError<A>> for Error<A> {
    fn from(error: SingleFieldError<A>) -> Self {
        Error::SingleField(error)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
/// Enumeration of errors that can occur during validation of two fields.
pub enum DoubleFieldError<FieldName = ()> {
    /// The provided entries should be distinct.
    NotDistinct(FieldName, FieldName),
    /// The provided left entry must be strictly smaller than the right entry.
    MustBeSmallerThan(FieldName, FieldName),
    /// The provided left entry must be strictly greater than the right entry.
    MustBeGreaterThan(FieldName, FieldName),
}

impl<A> From<DoubleFieldError<A>> for Error<A> {
    fn from(error: DoubleFieldError<A>) -> Self {
        Error::DoubleField(error)
    }
}

impl<A: core::fmt::Display> core::fmt::Display for SingleFieldError<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            SingleFieldError::EmptyText(field_name) => {
                write!(f, "Please provide a value for the {field_name} field.")
            }
            SingleFieldError::PaddedText(field_name) => {
                write!(
                    f,
                    "The {field_name} field contains leading or trailing spaces. Please remove them."
                )
            }
            SingleFieldError::ConsecutiveWhitespace(field_name) => {
                write!(
                    f,
                    "The {field_name} field contains consecutive whitespace characters. Please remove them."
                )
            }
            SingleFieldError::ControlCharacters(field_name) => {
                write!(f, "The {field_name} field contains control characters. Please remove them.")
            }
            SingleFieldError::InvalidMail(field_name) => {
                write!(
                    f,
                    "The {field_name} field contains an invalid email address. Please check and try again."
                )
            }
            SingleFieldError::InvalidFontAwesomeClass(field_name, icon) => {
                write!(
                    f,
                    "The {field_name} field contains an invalid Font Awesome class `{icon}`. Please check and try again."
                )
            }
            SingleFieldError::UnexpectedNegativeOrZeroValue(field_name) => {
                write!(f, "The {field_name} field must be a positive number greater than zero.")
            }
            SingleFieldError::UnexpectedNegativeValue(field_name) => {
                write!(f, "The {field_name} field must be a non-negative number (zero or greater).")
            }
            SingleFieldError::MustBeSmallerThan(field_name, expected_value) => {
                write!(f, "The {field_name} field must be smaller than {expected_value}.")
            }
            SingleFieldError::MustBeGreaterThan(field_name, expected_value) => {
                write!(f, "The {field_name} field must be greater than {expected_value}.")
            }
            SingleFieldError::InvalidCasCode(field_name, error) => {
                write!(f, "The {field_name} field must be a valid CAS code. Error: {error}")
            }
            SingleFieldError::InvalidMolecularFormula(field_name, error) => {
                write!(
                    f,
                    "The {field_name} field must be a valid molecular formula. Error: {error}"
                )
            }
            SingleFieldError::InvalidMediaType(field_name, error) => {
                write!(f, "The {field_name} field must be a valid media type. Error: {error}")
            }
        }
    }
}

impl<A: core::fmt::Display> core::fmt::Display for DoubleFieldError<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            DoubleFieldError::NotDistinct(left, right) => {
                write!(f, "The {left} and {right} fields must be distinct.")
            }
            DoubleFieldError::MustBeSmallerThan(left, right) => {
                write!(f, "The {left} field must be smaller than the {right} field.")
            }
            DoubleFieldError::MustBeGreaterThan(left, right) => {
                write!(f, "The {left} field must be greater than the {right} field.")
            }
        }
    }
}

impl DoubleFieldError {
    /// Renames the field names.
    pub fn rename_fields<FieldName>(
        self,
        left: FieldName,
        right: FieldName,
    ) -> DoubleFieldError<FieldName> {
        match self {
            DoubleFieldError::NotDistinct((), ()) => DoubleFieldError::NotDistinct(left, right),
            DoubleFieldError::MustBeSmallerThan((), ()) => {
                DoubleFieldError::MustBeSmallerThan(left, right)
            }
            DoubleFieldError::MustBeGreaterThan((), ()) => {
                DoubleFieldError::MustBeGreaterThan(left, right)
            }
        }
    }
}

impl<FieldName> DoubleFieldError<FieldName> {
    /// Converts the error into the provided new attribute type.
    pub fn into_field_name<F, NewFieldName>(self, convert: F) -> DoubleFieldError<NewFieldName>
    where
        F: Fn(FieldName) -> NewFieldName,
    {
        match self {
            DoubleFieldError::NotDistinct(left, right) => {
                DoubleFieldError::NotDistinct(convert(left), convert(right))
            }
            DoubleFieldError::MustBeSmallerThan(left, right) => {
                DoubleFieldError::MustBeSmallerThan(convert(left), convert(right))
            }
            DoubleFieldError::MustBeGreaterThan(left, right) => {
                DoubleFieldError::MustBeGreaterThan(convert(left), convert(right))
            }
        }
    }
}

#[cfg(feature = "diesel")]
impl TryFrom<diesel::result::Error> for Error {
    type Error = diesel::result::Error;

    fn try_from(_error: diesel::result::Error) -> Result<Self, Self::Error> {
        todo!()
    }
}
