use crate::codegen::diesel_codegen::tables::{
    instrument_locations::instrument_locations, users::users,
};
diesel::allow_tables_to_appear_in_same_query!(instrument_locations, users);
use crate::codegen::diesel_codegen::tables::instruments::instruments;
diesel::allow_tables_to_appear_in_same_query!(instrument_locations, instruments);
use crate::codegen::diesel_codegen::tables::rooms::rooms;
diesel::allow_tables_to_appear_in_same_query!(instrument_locations, rooms);
