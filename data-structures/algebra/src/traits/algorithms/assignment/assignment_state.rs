//! Enumeration for the LAPMOD assignment state, including the variants of:
//!
//! * `Unassigned`
//! * `Assigned`
//! * `Conflict`

#[derive(Debug, Clone, Copy, PartialEq)]
/// The LAPMOD assignment state.
pub enum AssignmentState<T> {
    /// The assignment is unassigned.
    Unassigned,
    /// The assignment is assigned.
    Assigned(T),
    /// The assignment is in conflict.
    Conflict(T),
}

impl<T> AssignmentState<T> {
    /// Returns true if the assignment is unassigned.
    pub fn is_unassigned(&self) -> bool {
        matches!(self, AssignmentState::Unassigned)
    }

    /// Returns true if the assignment is assigned.
    pub fn is_assigned(&self) -> bool {
        matches!(self, AssignmentState::Assigned(_))
    }
}
