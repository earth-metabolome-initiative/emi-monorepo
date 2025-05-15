use crate::codegen::diesel_codegen::tables::{organism_taxa::organism_taxa, organisms::organisms};
diesel::allow_tables_to_appear_in_same_query!(organism_taxa, organisms);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(organism_taxa, users);
use crate::codegen::diesel_codegen::tables::taxa::taxa;
diesel::allow_tables_to_appear_in_same_query!(organism_taxa, taxa);
