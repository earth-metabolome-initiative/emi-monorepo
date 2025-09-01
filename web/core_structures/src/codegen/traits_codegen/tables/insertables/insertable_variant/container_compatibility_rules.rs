impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRuleBuilder
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRule as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule,
    >,
    C: diesel::connection::LoadConnection,
{
    type Row = crate::codegen::structs_codegen::tables::container_compatibility_rules::ContainerCompatibilityRule;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRule;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRuleAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRule = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let container_model_id = self
            .container_model_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRuleAttributes::ContainerModelId,
                ),
            )?;
        let contained_asset_model = self
            .contained_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRuleAttributes::ContainedAssetModel,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRuleAttributes::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableContainerCompatibilityRuleAttributes::CreatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            container_model_id,
            contained_asset_model,
            quantity: self.quantity,
            created_by,
            created_at,
        })
    }
}
