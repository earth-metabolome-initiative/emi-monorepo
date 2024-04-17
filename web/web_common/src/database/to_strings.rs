//! Implementation of the convertion To String for different table structs.

use super::Taxa;

impl ToString for Taxa {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}