impl From<crate::codegen::structs_codegen::tables::team_states::TeamState> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::team_states::TeamState) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>> for super::Rows {
    fn from(value: Vec<crate::codegen::structs_codegen::tables::team_states::TeamState>) -> Self {
        super::Rows::TeamState(value)
    }
}
impl TryFrom<super::Rows> for Vec<crate::codegen::structs_codegen::tables::team_states::TeamState> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::TeamState(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
