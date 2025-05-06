use crate::codegen::diesel_codegen::tables::{
    organism_observations::organism_observations, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(organism_observations, users);
use crate::codegen::diesel_codegen::tables::projects::projects;
diesel::allow_tables_to_appear_in_same_query!(organism_observations, projects);
use crate::codegen::diesel_codegen::tables::observation_subjects::observation_subjects;
diesel::allow_tables_to_appear_in_same_query!(organism_observations, observation_subjects);
use crate::codegen::diesel_codegen::tables::organisms::organisms;
diesel::allow_tables_to_appear_in_same_query!(organism_observations, organisms);
