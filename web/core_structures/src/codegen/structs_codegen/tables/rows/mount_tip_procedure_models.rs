impl
    From<
        crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
    > for super::Rows
{
    fn from(
        value: crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
        >,
    ) -> Self {
        super::Rows::MountTipProcedureModel(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<
        crate::codegen::structs_codegen::tables::mount_tip_procedure_models::MountTipProcedureModel,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::MountTipProcedureModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
