impl<ProcedureTemplate> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder<
    ProcedureTemplate,
> {
    type Row = crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplate;
    type UserId = i32;
}
impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplate as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate,
    >,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute,
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
    crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateExtensionAttribute: From<
        <ProcedureTemplate as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<
        Self::Row,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute,
        >,
    > {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("pouring_procedure_templates");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplate = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        mut self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::InsertableVariant,
        web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute,
        >,
    > {
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_measured_with_model,
        ) = self.procedure_template_measured_with_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_measured_with_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateSettable>::measured_with_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_poured_from_model,
        ) = self.procedure_template_poured_from_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_poured_from_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateSettable>::poured_from_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            procedure_template_poured_into_model,
        ) = self.procedure_template_poured_into_model
        {
            let procedure_template_asset_models = crate::codegen::structs_codegen::tables::procedure_template_asset_models::ProcedureTemplateAssetModel::read(
                procedure_template_poured_into_model,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateSettable>::poured_into_model(
                self,
                procedure_template_asset_models.asset_model,
            )?;
        }
        let measured_with_model = self
            .measured_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::MeasuredWithModel,
                ),
            )?;
        let poured_from_model = self
            .poured_from_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::PouredFromModel,
                ),
            )?;
        let poured_into_model = self
            .poured_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::PouredIntoModel,
                ),
            )?;
        let liters = self
            .liters
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::Liters,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::Extension(
                        From::from(attribute),
                    )
                })
            })?;
        let procedure_template_measured_with_model = match self
            .procedure_template_measured_with_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_measured_with_model,
            ) => {
                procedure_template_measured_with_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_measured_with_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::ProcedureTemplateMeasuredWithModel,
                        )
                    })?;
                procedure_template_measured_with_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::ProcedureTemplateMeasuredWithModel,
                        )
                    })?
            }
        };
        let procedure_template_poured_from_model = match self
            .procedure_template_poured_from_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_poured_from_model,
            ) => {
                procedure_template_poured_from_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_poured_from_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::ProcedureTemplatePouredFromModel,
                        )
                    })?;
                procedure_template_poured_from_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::ProcedureTemplatePouredFromModel,
                        )
                    })?
            }
        };
        let procedure_template_poured_into_model = match self
            .procedure_template_poured_into_model
        {
            web_common_traits::database::IdOrBuilder::Id(id) => id,
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_template_poured_into_model,
            ) => {
                procedure_template_poured_into_model = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateAssetModelSettable>::procedure_template(
                        procedure_template_poured_into_model,
                        procedure_template,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::ProcedureTemplatePouredIntoModel,
                        )
                    })?;
                procedure_template_poured_into_model
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::PouringProcedureTemplateAttribute::ProcedureTemplatePouredIntoModel,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure_template,
            measured_with_model,
            procedure_template_measured_with_model,
            poured_from_model,
            procedure_template_poured_from_model,
            poured_into_model,
            procedure_template_poured_into_model,
            liters,
        })
    }
}
