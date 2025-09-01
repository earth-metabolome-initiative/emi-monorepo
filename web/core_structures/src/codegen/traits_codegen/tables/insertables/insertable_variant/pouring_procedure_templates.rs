impl<
    C: diesel::connection::LoadConnection,
    ProcedureTemplate,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateBuilder<
    ProcedureTemplate,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
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
    C: diesel::connection::LoadConnection,
    ProcedureTemplate: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelAttributes,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::pouring_procedure_templates::PouringProcedureTemplate;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplate;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplate = self
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
        let measured_with_model = self
            .measured_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::MeasuredWithModel,
                ),
            )?;
        let procedure_template_measured_with_model = self
            .procedure_template_measured_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::ProcedureTemplateMeasuredWithModel,
                ),
            )?;
        let poured_from_model = self
            .poured_from_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::PouredFromModel,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let procedure_template_poured_from_model = self
            .procedure_template_poured_from_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::ProcedureTemplatePouredFromModel,
                ),
            )?;
        let poured_into_model = self
            .poured_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::PouredIntoModel,
                ),
            )?;
        let procedure_template_poured_into_model = self
            .procedure_template_poured_into_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::ProcedureTemplatePouredIntoModel,
                ),
            )?;
        let liters = self
            .liters
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::Liters,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePouringProcedureTemplateExtensionAttributes::ProcedureTemplate(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAttributes::ProcedureTemplate,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure_template,
            measured_with_model,
            procedure_template_measured_with_model,
            poured_from_model,
            foreign_procedure_template,
            procedure_template_poured_from_model,
            poured_into_model,
            procedure_template_poured_into_model,
            liters,
        })
    }
}
