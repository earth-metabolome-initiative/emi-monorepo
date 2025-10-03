impl<Procedure> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureBuilder<
        Procedure,
    >
{
    type Row = crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute,
    >;
}
impl<Procedure> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureBuilder<
        Procedure,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedure;
}
#[cfg(feature = "backend")]
impl<Procedure> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedure,
        Row = crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("cleaning_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedure = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedureBuilder<
    Procedure,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCleaningProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure,
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
    Self: crate::codegen::structs_codegen::tables::insertables::CleaningProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate: web_common_traits::database::Read<
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
            let cleaning_procedure_templates = crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate::read(
                procedure_template,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CleaningProcedureSettable>::procedure_template_cleaned_with_model(
                self,
                cleaning_procedure_templates.procedure_template_cleaned_with_model,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CleaningProcedureSettable>::procedure_template_cleaned_model(
                self,
                cleaning_procedure_templates.procedure_template_cleaned_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_cleaned_with) = self
            .procedure_cleaned_with
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_cleaned_with,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CleaningProcedureSettable>::procedure_template_cleaned_with_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(procedure_cleaned) = self
            .procedure_cleaned
        {
            let procedure_assets = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_cleaned,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CleaningProcedureSettable>::procedure_template_cleaned_model(
                self,
                procedure_assets.procedure_template_asset_model,
            )?;
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let procedure_template_cleaned_with_model = self
            .procedure_template_cleaned_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute::ProcedureTemplateCleanedWithModel,
                ),
            )?;
        let procedure_template_cleaned_model = self
            .procedure_template_cleaned_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute::ProcedureTemplateCleanedModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_cleaned_with = match self.procedure_cleaned_with {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_cleaned_with,
            ) => {
                procedure_cleaned_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_cleaned_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute::ProcedureCleanedWith,
                        )
                    })?;
                procedure_cleaned_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute::ProcedureCleanedWith,
                        )
                    })?
            }
        };
        let procedure_cleaned = match self.procedure_cleaned {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(mut procedure_cleaned) => {
                procedure_cleaned = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_cleaned,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute::ProcedureCleaned,
                        )
                    })?;
                procedure_cleaned
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CleaningProcedureAttribute::ProcedureCleaned,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            procedure_template_cleaned_with_model,
            procedure_cleaned_with,
            procedure_template_cleaned_model,
            procedure_cleaned,
        })
    }
}
