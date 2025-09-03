impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure,
    >,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes,
    >,
    crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("aliquoting_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedure = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.procedure(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
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
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::Read;
        if let Some(procedure_template) = self.procedure_template {
            if let Some(aliquoting_procedure_templates) = crate::codegen::structs_codegen::tables::aliquoting_procedure_templates::AliquotingProcedureTemplate::read(
                procedure_template,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureBuildable>::foreign_procedure_template(
                    self,
                    aliquoting_procedure_templates.foreign_procedure_template,
                )?;
            }
        }
        if let Some(foreign_procedure) = self.foreign_procedure {
            if let Some(procedures) = crate::codegen::structs_codegen::tables::procedures::Procedure::read(
                foreign_procedure,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::AliquotingProcedureBuildable>::foreign_procedure_template(
                    self,
                    procedures.procedure_template,
                )?;
            }
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::ProcedureTemplate,
                ),
            )?;
        let foreign_procedure_template = self
            .foreign_procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::ForeignProcedureTemplate,
                ),
            )?;
        let foreign_procedure = self
            .foreign_procedure
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::ForeignProcedure,
                ),
            )?;
        let aliquoted_with = self
            .aliquoted_with
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::AliquotedWith,
                ),
            )?;
        let pipette_tip_model = self
            .pipette_tip_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::PipetteTipModel,
                ),
            )?;
        let aliquoted_from = self
            .aliquoted_from
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::AliquotedFrom,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureExtensionAttributes::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes::Procedure,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            foreign_procedure_template,
            foreign_procedure,
            aliquoted_with,
            pipette_tip_model,
            aliquoted_from,
        })
    }
}
