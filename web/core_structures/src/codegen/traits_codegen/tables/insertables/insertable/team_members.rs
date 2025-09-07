impl web_common_traits::database::Insertable for crate::TeamMember {
    type InsertableBuilder =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamMemberBuilder;
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableTeamMember;
}
