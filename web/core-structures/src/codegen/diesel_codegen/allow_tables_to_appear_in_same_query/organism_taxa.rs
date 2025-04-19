use crate::codegen::diesel_codegen::tables::{organism_taxa::organism_taxa, taxa::taxa};
diesel::allow_tables_to_appear_in_same_query!(organism_taxa, taxa);
use crate::codegen::diesel_codegen::tables::organisms::organisms;
diesel::allow_tables_to_appear_in_same_query!(organism_taxa, organisms);
use crate::codegen::diesel_codegen::tables::users::users;
diesel::allow_tables_to_appear_in_same_query!(organism_taxa, users);
