use std::fmt;

#[derive(Debug)]
pub enum SparqlClientError {
    QueryFileNotFound(String),
    QueryFileEmpty(String),
    InvalidResponseFormat(String),
    CsvWriteError(String),
    IoError(std::io::Error),
    ReqwestError(reqwest::Error),
    CsvError(csv::Error),
}

impl fmt::Display for SparqlClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SparqlClientError::QueryFileNotFound(msg) => write!(f, "Query file not found: {}", msg),
            SparqlClientError::QueryFileEmpty(msg) => write!(f, "Query file is empty: {}", msg),
            SparqlClientError::InvalidResponseFormat(msg) => write!(f, "Invalid response format: {}", msg),
            SparqlClientError::CsvWriteError(msg) => write!(f, "CSV write error: {}", msg),
            SparqlClientError::IoError(err) => write!(f, "IO error: {}", err),
            SparqlClientError::ReqwestError(err) => write!(f, "HTTP request error: {}", err),
            SparqlClientError::CsvError(err) => write!(f, "CSV error: {}", err),
        }
    }
}

// Implement std::error::Error
impl std::error::Error for SparqlClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SparqlClientError::IoError(err) => Some(err),
            SparqlClientError::ReqwestError(err) => Some(err),
            SparqlClientError::CsvError(err) => Some(err),
            _ => None,
        }
    }
}

// Implement From trait for standard errors
impl From<std::io::Error> for SparqlClientError {
    fn from(error: std::io::Error) -> Self {
        SparqlClientError::IoError(error)
    }
}

impl From<reqwest::Error> for SparqlClientError {
    fn from(error: reqwest::Error) -> Self {
        SparqlClientError::ReqwestError(error)
    }
}

impl From<csv::Error> for SparqlClientError {
    fn from(error: csv::Error) -> Self {
        SparqlClientError::CsvError(error)
    }
}
