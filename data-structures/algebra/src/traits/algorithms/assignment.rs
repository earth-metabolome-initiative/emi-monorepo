//! Submodule providing algorithms for solving the Assignment Problem.

mod hopcroft_karp;
pub use hopcroft_karp::*;
mod assignment_state;
pub use assignment_state::AssignmentState;
