impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder<
        Procedure,
    >
{
    type Row = crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder<
        Procedure,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedure,
        Row = crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("disposal_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedure = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
}
impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableDisposalProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::disposal_procedures::DisposalProcedure,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <Procedure as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
{
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let Some(procedure_template) = self.procedure_template {
            let disposal_procedure_templates = crate::codegen::structs_codegen::tables::disposal_procedure_templates::DisposalProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable>::procedure_template_disposed_asset_model(
                self,
                disposal_procedure_templates.procedure_template_disposed_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_disposed_asset) = self
            .procedure_disposed_asset
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_disposed_asset,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable>::disposed_asset(
                self,
                procedure_assets.asset,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::DisposalProcedureSettable>::procedure_template_disposed_asset_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let procedure_template_disposed_asset_model = self
            .procedure_template_disposed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureTemplateDisposedAssetModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_disposed_asset = match self.procedure_disposed_asset {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_disposed_asset,
            ) => {
                procedure_disposed_asset = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_disposed_asset,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureDisposedAsset,
                        )
                    })?;
                procedure_disposed_asset
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::DisposalProcedureAttribute::ProcedureDisposedAsset,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            disposed_asset: self.disposed_asset,
            procedure_template_disposed_asset_model,
            procedure_disposed_asset,
        })
    }
}
