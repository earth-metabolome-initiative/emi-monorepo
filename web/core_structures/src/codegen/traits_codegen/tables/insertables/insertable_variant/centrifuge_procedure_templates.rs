impl<ProcedureTemplate> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type Row = crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
    >;
}
impl<ProcedureTemplate> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplate;
}
#[cfg(feature = "backend")]
impl<ProcedureTemplate> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder<
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
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplate,
        Row = crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
        >,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateExtensionAttribute: From<
        <ProcedureTemplate as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("centrifuge_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplate = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::centrifuge_procedure_templates::CentrifugeProcedureTemplate,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attribute = crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelAttribute,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateExtensionAttribute: From<
        <ProcedureTemplate as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_centrifuged_with_model,
        ) = self.procedure_template_centrifuged_with_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_centrifuged_with_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateSettable>::centrifuged_with_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_centrifuged_container_model,
        ) = self.procedure_template_centrifuged_container_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_centrifuged_container_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateSettable>::centrifuged_container_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        let kelvin = self
            .kelvin
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::Kelvin,
                ),
            )?;
        let kelvin_tolerance_percentage = self
            .kelvin_tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::KelvinTolerancePercentage,
                ),
            )?;
        let seconds = self
            .seconds
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::Seconds,
                ),
            )?;
        let rotation_per_minute = self
            .rotation_per_minute
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::RotationPerMinute,
                ),
            )?;
        let centrifuged_with_model = self
            .centrifuged_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::CentrifugedWithModel,
                ),
            )?;
        let centrifuged_container_model = self
            .centrifuged_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::CentrifugedContainerModel,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::Extension(
                        From::from(attribute),
                    )
                })
            })?;
        let procedure_template_centrifuged_with_model = match self
            .procedure_template_centrifuged_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_centrifuged_with_model,
            ) => {
                procedure_template_centrifuged_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_centrifuged_with_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::ProcedureTemplateCentrifugedWithModel,
                        )
                    })?;
                procedure_template_centrifuged_with_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::ProcedureTemplateCentrifugedWithModel,
                        )
                    })?
            }
        };
        let procedure_template_centrifuged_container_model = match self
            .procedure_template_centrifuged_container_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_centrifuged_container_model,
            ) => {
                procedure_template_centrifuged_container_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_centrifuged_container_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::ProcedureTemplateCentrifugedContainerModel,
                        )
                    })?;
                procedure_template_centrifuged_container_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeProcedureTemplateAttribute::ProcedureTemplateCentrifugedContainerModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            kelvin,
            kelvin_tolerance_percentage,
            seconds,
            rotation_per_minute,
            centrifuged_with_model,
            procedure_template_centrifuged_with_model,
            centrifuged_container_model,
            procedure_template_centrifuged_container_model,
        })
    }
}
