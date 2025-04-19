//! Submodule providing enumeration on the kind of migration.

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration on the kind of migration.
pub enum MigrationKind {
    /// Up migration.
    Up,
    /// Down migration.
    Down,
}

impl Display for MigrationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationKind::Up => write!(f, "up"),
            MigrationKind::Down => write!(f, "down"),
        }
    }
}
