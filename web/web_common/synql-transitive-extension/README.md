# SynQL Transitive Extension

SynQL crate which provides traits to code-generate transitive extension marker traits for table inheritance hierarchies.

This crate generates `ExtensionOfTableName` traits that capture the transitive closure of table extension relationships, allowing simplified where clauses by replacing `T: ExtensionOf<A> + ExtensionOf<B> + ExtensionOf<C>` with just `T: ExtensionOfA`.
