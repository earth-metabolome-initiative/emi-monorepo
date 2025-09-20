impl<
    BallMillMachineModel,
    CommercialProduct,
> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {
    type Row = crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineModelAttribute,
    >;
}
impl<
    BallMillMachineModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModel;
}
#[cfg(feature = "backend")]
impl<
    BallMillMachineModel,
    CommercialProduct,
> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    BallMillMachineModel,
    CommercialProduct,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModel,
        Row = crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineModelAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_ball_mill_machine_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModel = self
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
    BallMillMachineModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModelBuilder<
    BallMillMachineModel,
    CommercialProduct,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBallMillMachineModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_ball_mill_machine_models::CommercialBallMillMachineModel,
    >,
    Self::Error: web_common_traits::database::FromExtension<
            <BallMillMachineModel as web_common_traits::database::TryInsertGeneric<
                C,
            >>::Error,
        >
        + web_common_traits::database::FromExtension<
            <CommercialProduct as web_common_traits::database::TryInsertGeneric<
                C,
            >>::Error,
        >,
    BallMillMachineModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        let ball_mill_machine_model = self
            .ball_mill_machine_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialBallMillMachineModelAttribute::BallMillMachineModel,
                ),
            )?;
        let id = if self.commercial_ball_mill_machine_models_id_fkey.is_complete() {
            let id = self
                .commercial_ball_mill_machine_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            let _ = self
                .commercial_ball_mill_machine_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            id
        } else {
            let id = self
                .commercial_ball_mill_machine_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            let _ = self
                .commercial_ball_mill_machine_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            ball_mill_machine_model,
        })
    }
}
