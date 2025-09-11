//! Submodule defining the `Hierarchy` struct, which represents the
//! hierarchy of procedure templates rooted at a given procedure template.

pub(crate) struct Hierarchy {
	/// The hierarchy of procedure templates, rooted at the procedure template
	/// being built, and including all its sub-procedure templates.
	hierarchy: DiGraph<Rc<ProcedureTemplate>>
}