//! Crate providing common validation errors.

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum SingleFieldError<FieldName = ()> {
    /// The provided text is empty.
    EmptyText(FieldName),
    /// The provided mail address is invalid.
    InvalidMail(FieldName),
    /// The float is not strictly positive (0.0, ...]
    UnexpectedNegativeOrZeroValue(FieldName),
    /// The float is not strictly greater than the expected amount.
    MustBeSmallerThan(FieldName, f64),
    /// The float is not strictly smaller than the expected amount.
    MustBeGreaterThan(FieldName, f64),
}

impl SingleFieldError {
    /// Renames the field name.
    pub fn rename_field<FieldName>(self, field_name: FieldName) -> SingleFieldError<FieldName> {
        match self {
            SingleFieldError::EmptyText(_) => SingleFieldError::EmptyText(field_name),
            SingleFieldError::InvalidMail(_) => SingleFieldError::InvalidMail(field_name),
            SingleFieldError::UnexpectedNegativeOrZeroValue(_) => {
                SingleFieldError::UnexpectedNegativeOrZeroValue(field_name)
            }
            SingleFieldError::MustBeSmallerThan(_, expected_value) => {
                SingleFieldError::MustBeSmallerThan(field_name, expected_value)
            }
            SingleFieldError::MustBeGreaterThan(_, expected_value) => {
                SingleFieldError::MustBeGreaterThan(field_name, expected_value)
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
            | SingleFieldError::InvalidMail(_)
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

#[derive(Debug, Clone, PartialEq)]
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
            SingleFieldError::InvalidMail(field_name) => {
                write!(
                    f,
                    "The {field_name} field contains an invalid email address. Please check and try again."
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
