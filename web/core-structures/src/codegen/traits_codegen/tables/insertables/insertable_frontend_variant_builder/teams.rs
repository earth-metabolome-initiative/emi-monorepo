#[cfg(feature = "postgres")]
impl web_common_traits::prelude::InsertableBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder {
    type Row = crate::codegen::structs_codegen::tables::teams::Team;
    type Product = crate::codegen::structs_codegen::tables::insertables::InsertableTeam;
}
