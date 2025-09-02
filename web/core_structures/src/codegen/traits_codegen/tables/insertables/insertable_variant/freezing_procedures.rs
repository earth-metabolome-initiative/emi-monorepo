impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure,
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
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureAttributes,
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
            "freezing_procedures",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedure = self
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
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureAttributes::ProcedureTemplate,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let foreign_procedure = self
            .foreign_procedure
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureAttributes::ForeignProcedure,
                ),
            )?;
        let frozen_container = self
            .frozen_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureAttributes::FrozenContainer,
                ),
            )?;
        let frozen_with_model = self
            .frozen_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureAttributes::FrozenWithModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFreezingProcedureExtensionAttributes::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes::Procedure,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            foreign_procedure_template,
            foreign_procedure,
            frozen_container,
            frozen_with: self.frozen_with,
            frozen_with_model,
        })
    }
}
