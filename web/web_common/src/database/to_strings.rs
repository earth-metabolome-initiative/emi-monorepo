//! Implementation of the convertion To String for different table structs.

use super::{ProjectState, Taxa};

// impl ToString for Taxa {
//     fn to_string(&self) -> String {
//         self.name.clone()
//     }
// }

impl ToString for ProjectState {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}