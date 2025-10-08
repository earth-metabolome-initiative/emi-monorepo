//! Implementation for the `SQL Doc Error` Enum to handle errors when parsing
//! SQL comments for documentation

/// Enum for the SQL Doc Errors
pub enum SqlDocError {
    /// Error for Missing Table Comment
    MissingTableComment,
    /// Error for Missing Column Comments
    MissingColumnComment,
}
