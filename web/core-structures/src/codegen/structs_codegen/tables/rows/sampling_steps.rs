impl From<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep> for super::Rows {
    fn from(value: crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep) -> Self {
        Self::from(std::rc::Rc::new(value))
    }
}
impl From<std::rc::Rc<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>
    for super::Rows
{
    fn from(
        value: std::rc::Rc<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>
    for super::Rows
{
    fn from(
        value: Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
    ) -> Self {
        Self::from(value.into_iter().map(std::rc::Rc::new).collect::<Vec<_>>())
    }
}
impl From<Vec<std::rc::Rc<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>>
    for super::Rows
{
    fn from(
        value: Vec<
            std::rc::Rc<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
        >,
    ) -> Self {
        Self::from(std::rc::Rc::new(value))
    }
}
impl From<std::rc::Rc<Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>>
    for super::Rows
{
    fn from(
        value: std::rc::Rc<
            Vec<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>,
        >,
    ) -> Self {
        Self::from(std::rc::Rc::new(
            value.iter().cloned().map(std::rc::Rc::new).collect::<Vec<_>>(),
        ))
    }
}
impl
    From<
        std::rc::Rc<
            Vec<std::rc::Rc<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>,
        >,
    > for super::Rows
{
    fn from(
        value: std::rc::Rc<
            Vec<std::rc::Rc<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>,
        >,
    ) -> Self {
        super::Rows::SamplingStep(value)
    }
}
impl TryFrom<super::Rows>
    for std::rc::Rc<
        Vec<std::rc::Rc<crate::codegen::structs_codegen::tables::sampling_steps::SamplingStep>>,
    >
{
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::SamplingStep(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
