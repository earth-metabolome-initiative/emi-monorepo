impl<ProcedureTemplate> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type Row = crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute,
    >;
}
impl<ProcedureTemplate> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplate;
}
#[cfg(feature = "backend")]
impl<ProcedureTemplate> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplate,
        Row = crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("photograph_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplate = self
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
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::photograph_procedure_templates::PhotographProcedureTemplate,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <ProcedureTemplate as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::DispatchableInsertableVariant<
        C,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
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
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_photographed_with_model,
        ) = self.procedure_template_photographed_with_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_photographed_with_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateSettable>::photographed_with_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_photographed_asset_model,
        ) = self.procedure_template_photographed_asset_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_photographed_asset_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateSettable>::photographed_asset_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_photograph_model,
        ) = self.procedure_template_photograph_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_photograph_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateSettable>::photograph_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        let photographed_with_model = self
            .photographed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::PhotographedWithModel,
                ),
            )?;
        let photographed_asset_model = self
            .photographed_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::PhotographedAssetModel,
                ),
            )?;
        let photograph_model = self
            .photograph_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::PhotographModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        let procedure_template_photographed_with_model = match self
            .procedure_template_photographed_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_photographed_with_model,
            ) => {
                procedure_template_photographed_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_photographed_with_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::ProcedureTemplatePhotographedWithModel,
                        )
                    })?;
                procedure_template_photographed_with_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::ProcedureTemplatePhotographedWithModel,
                        )
                    })?
            }
        };
        let procedure_template_photographed_asset_model = match self
            .procedure_template_photographed_asset_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_photographed_asset_model,
            ) => {
                procedure_template_photographed_asset_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_photographed_asset_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::ProcedureTemplatePhotographedAssetModel,
                        )
                    })?;
                procedure_template_photographed_asset_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::ProcedureTemplatePhotographedAssetModel,
                        )
                    })?
            }
        };
        let procedure_template_photograph_model = match self
            .procedure_template_photograph_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_photograph_model,
            ) => {
                procedure_template_photograph_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_photograph_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::ProcedureTemplatePhotographModel,
                        )
                    })?;
                procedure_template_photograph_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PhotographProcedureTemplateAttribute::ProcedureTemplatePhotographModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            photographed_with_model,
            procedure_template_photographed_with_model,
            photographed_asset_model,
            procedure_template_photographed_asset_model,
            photograph_model,
            procedure_template_photograph_model,
        })
    }
}
