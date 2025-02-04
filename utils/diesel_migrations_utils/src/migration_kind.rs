//! Submodule providing enumeration on the kind of migration.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Enumeration on the kind of migration.
pub enum MigrationKind {
	/// Up migration.
	Up,
	/// Down migration.
	Down
}