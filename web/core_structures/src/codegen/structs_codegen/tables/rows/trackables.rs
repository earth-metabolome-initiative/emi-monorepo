impl From<crate::codegen::structs_codegen::tables::trackables::Trackable> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::trackables::Trackable) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::trackables::Trackable>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::trackables::Trackable>) -> Self {
        super::Rows::Trackable(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::trackables::Trackable> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::Trackable(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
