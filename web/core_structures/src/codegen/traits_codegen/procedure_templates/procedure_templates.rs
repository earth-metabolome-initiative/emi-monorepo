impl<C> web_common_traits::prelude::ProcedureTemplate<C> for crate::ProcedureTemplate
where
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    C: diesel::connection::LoadConnection,
    Self: web_common_traits::database::MostConcreteVariant<
        C,
        Variant = crate::codegen::structs_codegen::tables::most_concrete_variants::ProcedureTemplateDAG,
    >,
    crate::ProcedureTemplateAssetModel: diesel::associations::BelongsTo<
        crate::ProcedureTemplate,
    >,
    for<'a> <crate::ProcedureTemplateAssetModel as diesel::BelongingToDsl<
        &'a crate::ProcedureTemplate,
    >>::Output: diesel::query_dsl::LoadQuery<'a, C, crate::ProcedureTemplateAssetModel>,
{
    type Procedure = crate::Procedure;
    type ProcedureTemplateAssetModel = crate::ProcedureTemplateAssetModel;
    fn procedure_template_id(&self) -> i32 {
        self.procedure_template
    }
    fn procedure_template_asset_models(
        &self,
        conn: &mut C,
    ) -> Result<Vec<Self::ProcedureTemplateAssetModel>, diesel::result::Error> {
        use web_common_traits::database::MostConcreteVariant;
        self.most_concrete_variant(conn)?.procedure_template_asset_models(conn)
    }
}
