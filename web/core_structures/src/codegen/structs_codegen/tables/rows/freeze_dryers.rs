impl From<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer>,
    ) -> Self {
        super::Rows::FreezeDryer(value)
    }
}
impl TryFrom<super::Rows>
    for Vec<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer>
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::FreezeDryer(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
