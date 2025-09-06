impl<
    C: diesel::connection::LoadConnection,
    Procedure,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureBuilder<
    Procedure,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedure as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure,
    >,
    C: diesel::connection::LoadConnection,
    Procedure: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset: web_common_traits::database::Read<
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
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedure;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute,
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
        self.set_most_concrete_table("ball_mill_procedures");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedure = self
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
        use web_common_traits::database::TryInsertGeneric;
        use web_common_traits::database::Read;
        if let Some(procedure_template) = self.procedure_template {
            if let Some(ball_mill_procedure_templates) = crate::codegen::structs_codegen::tables::ball_mill_procedure_templates::BallMillProcedureTemplate::read(
                procedure_template,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::procedure_template_milled_with_model(
                    self,
                    ball_mill_procedure_templates.procedure_template_milled_with_model,
                )?;
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::procedure_template_milled_container_model(
                    self,
                    ball_mill_procedure_templates
                        .procedure_template_milled_container_model,
                )?;
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::procedure_template_bead_model(
                    self,
                    ball_mill_procedure_templates.procedure_template_bead_model,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(Some(procedure_bead)) = self
            .procedure_bead
        {
            if let Some(procedure_assets) = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_bead,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::bead_model(
                    self,
                    procedure_assets.asset_model,
                )?;
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::procedure_template_bead_model(
                    self,
                    procedure_assets.procedure_template_asset_model,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            Some(procedure_milled_with),
        ) = self.procedure_milled_with
        {
            if let Some(procedure_assets) = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_milled_with,
                conn,
            )? {
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::milled_with(
                    self,
                    procedure_assets.asset,
                )?;
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::milled_with_model(
                    self,
                    procedure_assets.asset_model,
                )?;
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::procedure_template_milled_with_model(
                    self,
                    procedure_assets.procedure_template_asset_model,
                )?;
            }
        }
        if let web_common_traits::database::IdOrBuilder::Id(
            Some(procedure_milled_container),
        ) = self.procedure_milled_container
        {
            if let Some(procedure_assets) = crate::codegen::structs_codegen::tables::procedure_assets::ProcedureAsset::read(
                procedure_milled_container,
                conn,
            )? {
                if let Some(asset) = procedure_assets.asset {
                    self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::milled_container(
                        self,
                        asset,
                    )?;
                }
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::milled_container_model(
                    self,
                    procedure_assets.asset_model,
                )?;
                self = <Self as crate::codegen::structs_codegen::tables::insertables::BallMillProcedureSettable>::procedure_template_milled_container_model(
                    self,
                    procedure_assets.procedure_template_asset_model,
                )?;
            }
        }
        let procedure_template = self
            .procedure_template
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureTemplate,
                ),
            )?;
        let bead_model = self
            .bead_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::BeadModel,
                ),
            )?;
        let procedure_template_bead_model = self
            .procedure_template_bead_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureTemplateBeadModel,
                ),
            )?;
        let milled_with_model = self
            .milled_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::MilledWithModel,
                ),
            )?;
        let procedure_template_milled_with_model = self
            .procedure_template_milled_with_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureTemplateMilledWithModel,
                ),
            )?;
        let milled_container = self
            .milled_container
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::MilledContainer,
                ),
            )?;
        let milled_container_model = self
            .milled_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::MilledContainerModel,
                ),
            )?;
        let procedure_template_milled_container_model = self
            .procedure_template_milled_container_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureTemplateMilledContainerModel,
                ),
            )?;
        let procedure = self
            .procedure
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::BallMillProcedureExtensionAttribute::Procedure(
                        crate::codegen::structs_codegen::tables::insertables::ProcedureAttribute::Procedure,
                    ),
                ))
            })?;
        let procedure_bead = match self.procedure_bead {
            web_common_traits::database::IdOrBuilder::Id(id) => {
                id.mint_primary_key(user_id, conn)
                    .map_err(|_| {
                        common_traits::prelude::BuilderError::IncompleteBuild(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureBead(
                                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
                            ),
                        )
                    })?
            }
            web_common_traits::database::IdOrBuilder::Builder(mut procedure_bead) => {
                procedure_bead = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_bead,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureBead,
                        )
                    })?;
                procedure_bead
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureBead,
                        )
                    })?
            }
        };
        let procedure_milled_with = match self.procedure_milled_with {
            web_common_traits::database::IdOrBuilder::Id(id) => {
                id.mint_primary_key(user_id, conn)
                    .map_err(|_| {
                        common_traits::prelude::BuilderError::IncompleteBuild(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureMilledWith(
                                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
                            ),
                        )
                    })?
            }
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_milled_with,
            ) => {
                procedure_milled_with = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_milled_with,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureMilledWith,
                        )
                    })?;
                procedure_milled_with
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureMilledWith,
                        )
                    })?
            }
        };
        let procedure_milled_container = match self.procedure_milled_container {
            web_common_traits::database::IdOrBuilder::Id(id) => {
                id.mint_primary_key(user_id, conn)
                    .map_err(|_| {
                        common_traits::prelude::BuilderError::IncompleteBuild(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureMilledContainer(
                                crate::codegen::structs_codegen::tables::insertables::ProcedureAssetAttribute::Id,
                            ),
                        )
                    })?
            }
            web_common_traits::database::IdOrBuilder::Builder(
                mut procedure_milled_container,
            ) => {
                procedure_milled_container = <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAssetBuilder as crate::codegen::structs_codegen::tables::insertables::ProcedureAssetSettable>::procedure(
                        procedure_milled_container,
                        procedure,
                    )
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureMilledContainer,
                        )
                    })?;
                procedure_milled_container
                    .mint_primary_key(user_id, conn)
                    .map_err(|err| {
                        err.into_field_name(
                            crate::codegen::structs_codegen::tables::insertables::BallMillProcedureAttribute::ProcedureMilledContainer,
                        )
                    })?
            }
        };
        Ok(Self::InsertableVariant {
            procedure,
            procedure_template,
            bead_model,
            procedure_template_bead_model,
            procedure_bead,
            milled_with_model,
            procedure_template_milled_with_model,
            procedure_milled_with,
            milled_with: self.milled_with,
            milled_container,
            milled_container_model,
            procedure_template_milled_container_model,
            procedure_milled_container,
        })
    }
}
