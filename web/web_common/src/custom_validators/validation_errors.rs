use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

pub trait ValidationErrorToString {
    fn convert_to_string(&self) -> Vec<String>;
}

impl ValidationErrorToString for ValidationError {
    fn convert_to_string(&self) -> Vec<String> {
        vec![self.message
            .clone()
            .map(|message| format!("{}", message))
            .unwrap_or_else(|| self.code.to_string())]
    }
}

impl ValidationErrorToString for ValidationErrors {
    fn convert_to_string(&self) -> Vec<String> {
        self.errors()
            .iter()
            .flat_map(|(_, errors)| {
                match errors {
                    ValidationErrorsKind::Field(errors) => errors
                        .iter()
                        .flat_map(|error| error.convert_to_string())
                        .collect::<Vec<String>>(),
                    ValidationErrorsKind::List(errors) => errors
                        .iter()
                        .flat_map(|(_, error)| error.convert_to_string())
                        .collect::<Vec<String>>(),
                    ValidationErrorsKind::Struct(errors) => errors.convert_to_string()
                }
            })
            .collect::<Vec<String>>()
    }
}

pub trait TryFromString: Sized {
    fn try_from_string(value: String) -> Result<Self, Vec<String>>;
}

impl TryFromString for String {
    fn try_from_string(value: String) -> Result<Self, Vec<String>> {
        Ok(value)
    }
}