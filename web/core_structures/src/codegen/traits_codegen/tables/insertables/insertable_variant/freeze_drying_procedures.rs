impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure,
    >,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureAttributes,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureBuildable>::most_concrete_table(
            self,
            "freeze_drying_procedures",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedure = self
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
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureAttributes::ProcedureTemplate,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let foreign_procedure = self
            .foreign_procedure
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureAttributes::ForeignProcedure,
                ),
            )?;
        let freeze_dryed_container = self
            .freeze_dryed_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureAttributes::FreezeDryedContainer,
                ),
            )?;
        let freeze_dryed_with_model = self
            .freeze_dryed_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureAttributes::FreezeDryedWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezeDryingProcedureExtensionAttributes::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes::Procedure,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            foreign_procedure_template,
            foreign_procedure,
            freeze_dryed_container,
            freeze_dryed_with: self.freeze_dryed_with,
            freeze_dryed_with_model,
        })
    }
}
