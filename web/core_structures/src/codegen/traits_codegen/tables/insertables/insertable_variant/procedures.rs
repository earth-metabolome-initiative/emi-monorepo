impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder
{
    type Row = crate::codegen::structs_codegen::tables::procedures::Procedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedure;
}
#[cfg(feature = "backend")]
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableProcedure,
        Row = crate::codegen::structs_codegen::tables::procedures::Procedure,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Updatable<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::Updatable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableProcedure = self
            .try_insert(user_id, conn)?;
        if let Some(parent) = insertable_struct.parent_procedure_template(conn)?
            && !parent.can_update(user_id, conn)? {
                return Err(
                    generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                        .into(),
                );
            }
        if let Some(parent) = insertable_struct.predecessor_procedure_template(conn)?
            && !parent.can_update(user_id, conn)? {
                return Err(
                    generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                        .into(),
                );
            }
        if !insertable_struct.procedure_template(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::procedures::Procedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::procedures::Procedure,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::ProcedureSettable<
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute,
        >,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate: web_common_traits::database::Updatable<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedures::Procedure: web_common_traits::database::Read<
        C,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn try_insert(
        mut self,
        _user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::Read;
        if let Some(parent_procedure) = self.parent_procedure {
            let procedures = crate::codegen::structs_codegen::tables::procedures::Procedure::read(
                parent_procedure,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::parent_procedure_template(
                self,
                procedures.procedure_template,
            )?;
        }
        if let Some(predecessor_procedure) = self.predecessor_procedure {
            let procedures = crate::codegen::structs_codegen::tables::procedures::Procedure::read(
                predecessor_procedure,
                conn,
            )?;
            self = <Self as crate::codegen::structs_codegen::tables::insertables::ProcedureSettable>::predecessor_procedure_template(
                self,
                procedures.procedure_template,
            )?;
        }
        let procedure = self
            .procedure
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                ),
            )?;
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let most_concrete_table = self
            .most_concrete_table
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::MostConcreteTable,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::CreatedAt,
                ),
            )?;
        let updated_by = self
            .updated_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::UpdatedBy,
                ),
            )?;
        let updated_at = self
            .updated_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::UpdatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            parent_procedure: self.parent_procedure,
            parent_procedure_template: self.parent_procedure_template,
            predecessor_procedure: self.predecessor_procedure,
            predecessor_procedure_template: self.predecessor_procedure_template,
            most_concrete_table,
            created_by,
            created_at,
            updated_by,
            updated_at,
        })
    }
}
