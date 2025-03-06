#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Insertable
    for crate::codegen::structs_codegen::tables::teams::Team
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableTeam;
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamBuilder;
}
