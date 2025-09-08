impl<
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelBuilder<
    PhysicalAssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::PackagingModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModel as diesel::Insertable<
            <crate::PackagingModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::PackagingModel>,
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::PackagingModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("packaging_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModel = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::PackagingModelAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::PackagingModelExtensionAttribute::PhysicalAssetModel(
                        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant { id })
    }
}
