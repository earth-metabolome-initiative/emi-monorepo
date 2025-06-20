//! Submodule defining the sample collection procedures in the
//! database.

mod full_organism_collection_procedure;
mod part_of_organism_collection_procedure;
pub(crate) use full_organism_collection_procedure::init_full_organism_collection;
pub(crate) use part_of_organism_collection_procedure::{
    CONICAL_TUBE_BOX, init_part_of_organism_collection,
};
