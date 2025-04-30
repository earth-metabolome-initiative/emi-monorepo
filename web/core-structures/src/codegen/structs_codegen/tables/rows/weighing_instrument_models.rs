impl From<
    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
> for super::Rows {
    fn from(
        value: crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    ) -> Self {
        Self::from(std::rc::Rc::new(value))
    }
}
impl From<
    std::rc::Rc<
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    >,
> for super::Rows {
    fn from(
        value: std::rc::Rc<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    ) -> Self {
        Self::from(vec![value])
    }
}
impl From<
    Vec<
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    ) -> Self {
        Self::from(value.into_iter().map(std::rc::Rc::new).collect::<Vec<_>>())
    }
}
impl From<
    Vec<
        std::rc::Rc<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    >,
> for super::Rows {
    fn from(
        value: Vec<
            std::rc::Rc<
                crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
            >,
        >,
    ) -> Self {
        Self::from(std::rc::Rc::new(value))
    }
}
impl From<
    std::rc::Rc<
        Vec<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    >,
> for super::Rows {
    fn from(
        value: std::rc::Rc<
            Vec<
                crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
            >,
        >,
    ) -> Self {
        Self::from(
            std::rc::Rc::new(
                value.iter().cloned().map(std::rc::Rc::new).collect::<Vec<_>>(),
            ),
        )
    }
}
impl From<
    std::rc::Rc<
        Vec<
            std::rc::Rc<
                crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
            >,
        >,
    >,
> for super::Rows {
    fn from(
        value: std::rc::Rc<
            Vec<
                std::rc::Rc<
                    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
                >,
            >,
        >,
    ) -> Self {
        super::Rows::WeighingInstrumentModel(value)
    }
}
impl TryFrom<super::Rows>
for std::rc::Rc<
    Vec<
        std::rc::Rc<
            crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
        >,
    >,
> {
    type Error = std::convert::Infallible;
    fn try_from(value: super::Rows) -> Result<Self, Self::Error> {
        match value {
            super::Rows::WeighingInstrumentModel(v) => Ok(v),
            value => unreachable!("Unexpected variant in Rows enum: {value:?}"),
        }
    }
}
