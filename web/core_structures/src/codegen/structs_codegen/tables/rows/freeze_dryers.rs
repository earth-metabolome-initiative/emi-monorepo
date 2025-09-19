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
impl From<super::Rows>
    for Option<Vec<crate::codegen::structs_codegen::tables::freeze_dryers::FreezeDryer>>
{
    fn from(value: super::Rows) -> Self {
        match value {
            super::Rows::FreezeDryer(v) => Some(v),
            _ => None,
        }
    }
}
