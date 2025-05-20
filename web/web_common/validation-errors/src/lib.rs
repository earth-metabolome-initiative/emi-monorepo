//! Crate providing common validation errors.

use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
/// Enumeration of errors that can occur during validation.
pub enum Error<FieldName = ()> {
    /// Single field errors.
    SingleField(SingleFieldError<FieldName>),
    /// Double field errors.
    DoubleField(DoubleFieldError<FieldName>),
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
    InvalidFontAwesomeClass(FieldName),
    /// The float is not strictly positive (0.0, ...]
    UnexpectedNegativeOrZeroValue(FieldName),
    /// The float is not strictly greater than the expected amount.
    MustBeSmallerThan(FieldName, f64),
    /// The float is not strictly smaller than the expected amount.
    MustBeGreaterThan(FieldName, f64),
    /// The provided text is not a valid instrument category.
    UnknownInstrumentCategory(FieldName),
}

impl SingleFieldError {
    /// Renames the field name.
    pub fn rename_field<FieldName>(self, field_name: FieldName) -> SingleFieldError<FieldName> {
        match self {
            SingleFieldError::EmptyText(_) => SingleFieldError::EmptyText(field_name),
            SingleFieldError::PaddedText(_) => SingleFieldError::PaddedText(field_name),
            SingleFieldError::ConsecutiveWhitespace(_) => {
                SingleFieldError::ConsecutiveWhitespace(field_name)
            }
            SingleFieldError::ControlCharacters(_) => {
                SingleFieldError::ControlCharacters(field_name)
            }
            SingleFieldError::InvalidMail(_) => SingleFieldError::InvalidMail(field_name),
            SingleFieldError::InvalidFontAwesomeClass(_) => {
                SingleFieldError::InvalidFontAwesomeClass(field_name)
            }
            SingleFieldError::UnexpectedNegativeOrZeroValue(_) => {
                SingleFieldError::UnexpectedNegativeOrZeroValue(field_name)
            }
            SingleFieldError::MustBeSmallerThan(_, expected_value) => {
                SingleFieldError::MustBeSmallerThan(field_name, expected_value)
            }
            SingleFieldError::MustBeGreaterThan(_, expected_value) => {
                SingleFieldError::MustBeGreaterThan(field_name, expected_value)
            }
            SingleFieldError::UnknownInstrumentCategory(_) => {
                SingleFieldError::UnknownInstrumentCategory(field_name)
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
            SingleFieldError::MustBeSmallerThan(_, _) => {
                DoubleFieldError::MustBeSmallerThan(left, right)
            }
            SingleFieldError::MustBeGreaterThan(_, _) => {
                DoubleFieldError::MustBeGreaterThan(left, right)
            }
            SingleFieldError::EmptyText(_)
            | SingleFieldError::PaddedText(_)
            | SingleFieldError::ConsecutiveWhitespace(_)
            | SingleFieldError::ControlCharacters(_)
            | SingleFieldError::InvalidFontAwesomeClass(_)
            | SingleFieldError::InvalidMail(_)
            | SingleFieldError::UnknownInstrumentCategory(_)
            | SingleFieldError::UnexpectedNegativeOrZeroValue(_) => {
                unimplemented!("Cannot convert the variant error into a double field error.")
            }
        }
    }
}

impl<A> From<SingleFieldError<A>> for Error<A> {
    fn from(error: SingleFieldError<A>) -> Self {
        Error::SingleField(error)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
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
            SingleFieldError::InvalidFontAwesomeClass(field_name) => {
                write!(
                    f,
                    "The {field_name} field contains an invalid Font Awesome class. Please check and try again."
                )
            }
            SingleFieldError::UnexpectedNegativeOrZeroValue(field_name) => {
                write!(f, "The {field_name} field must be a positive number greater than zero.")
            }
            SingleFieldError::MustBeSmallerThan(field_name, expected_value) => {
                write!(f, "The {field_name} field must be smaller than {expected_value}.")
            }
            SingleFieldError::MustBeGreaterThan(field_name, expected_value) => {
                write!(f, "The {field_name} field must be greater than {expected_value}.")
            }
            SingleFieldError::UnknownInstrumentCategory(field_name) => {
                write!(f, "The {field_name} field must be a valid instrument category.")
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
            DoubleFieldError::NotDistinct(_, _) => DoubleFieldError::NotDistinct(left, right),
            DoubleFieldError::MustBeSmallerThan(_, _) => {
                DoubleFieldError::MustBeSmallerThan(left, right)
            }
            DoubleFieldError::MustBeGreaterThan(_, _) => {
                DoubleFieldError::MustBeGreaterThan(left, right)
            }
        }
    }
}

impl TryFrom<diesel::result::Error> for Error {
    type Error = diesel::result::Error;

    fn try_from(_error: diesel::result::Error) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<Infallible> for Error {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to Error.")
    }
}

impl From<Infallible> for SingleFieldError {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to SingleFieldError.")
    }
}

impl From<Infallible> for DoubleFieldError {
    fn from(_error: Infallible) -> Self {
        unreachable!("Infallible cannot be converted to DoubleFieldError.")
    }
}

impl From<instrument_categories::errors::UnknownInstrumentCategory> for Error {
    fn from(error: instrument_categories::errors::UnknownInstrumentCategory) -> Self {
        Self::SingleField(error.into())
    }
}

impl From<instrument_categories::errors::UnknownInstrumentCategory> for SingleFieldError {
    fn from(_error: instrument_categories::errors::UnknownInstrumentCategory) -> Self {
        Self::UnknownInstrumentCategory(())
    }
}
