#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFractioningProcedureModelExtensionAttributes {
    ProcedureModel(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    ),
}
impl core::fmt::Display for InsertableFractioningProcedureModelExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::ProcedureModel(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableFractioningProcedureModelAttributes {
    Extension(InsertableFractioningProcedureModelExtensionAttributes),
    ProcedureModelId,
    Kilograms,
    TolerancePercentage,
    WeighedWith,
    ProcedureWeighedWith(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    ProcedureFragmentSource(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
    FragmentPlacedInto,
    ProcedureFragmentPlacedInto(
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
    ),
}
impl core::str::FromStr for InsertableFractioningProcedureModelAttributes {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Kilograms" => Ok(Self::Kilograms),
            "TolerancePercentage" => Ok(Self::TolerancePercentage),
            "WeighedWith" => Ok(Self::WeighedWith),
            "ProcedureWeighedWith" => {
                Ok(
                    Self::ProcedureWeighedWith(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            "ProcedureFragmentSource" => {
                Ok(
                    Self::ProcedureFragmentSource(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            "FragmentPlacedInto" => Ok(Self::FragmentPlacedInto),
            "ProcedureFragmentPlacedInto" => {
                Ok(
                    Self::ProcedureFragmentPlacedInto(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            "kilograms" => Ok(Self::Kilograms),
            "tolerance_percentage" => Ok(Self::TolerancePercentage),
            "weighed_with" => Ok(Self::WeighedWith),
            "procedure_weighed_with" => {
                Ok(
                    Self::ProcedureWeighedWith(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            "procedure_fragment_source" => {
                Ok(
                    Self::ProcedureFragmentSource(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            "fragment_placed_into" => Ok(Self::FragmentPlacedInto),
            "procedure_fragment_placed_into" => {
                Ok(
                    Self::ProcedureFragmentPlacedInto(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::Id,
                    ),
                )
            }
            _ => {
                Err(
                    web_common_traits::database::InsertError::UnknownAttribute(
                        s.to_owned(),
                    ),
                )
            }
        }
    }
}
impl core::fmt::Display for InsertableFractioningProcedureModelAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureModelId => write!(f, "procedure_model_id"),
            Self::Kilograms => write!(f, "kilograms"),
            Self::TolerancePercentage => write!(f, "tolerance_percentage"),
            Self::WeighedWith => write!(f, "weighed_with"),
            Self::ProcedureWeighedWith(e) => write!(f, "{e}"),
            Self::ProcedureFragmentSource(e) => write!(f, "{e}"),
            Self::FragmentPlacedInto => write!(f, "fragment_placed_into"),
            Self::ProcedureFragmentPlacedInto(e) => write!(f, "{e}"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::fractioning_procedure_models::fractioning_procedure_models
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningProcedureModel {
    pub(crate) procedure_model_id: i32,
    pub(crate) kilograms: f32,
    pub(crate) tolerance_percentage: f32,
    pub(crate) weighed_with: ::rosetta_uuid::Uuid,
    pub(crate) procedure_weighed_with: i32,
    pub(crate) procedure_fragment_source: i32,
    pub(crate) fragment_placed_into: ::rosetta_uuid::Uuid,
    pub(crate) procedure_fragment_placed_into: i32,
}
impl InsertableFractioningProcedureModel {
    pub fn procedure_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel::table(),
                self.procedure_model_id,
            ),
            conn,
        )
    }
    pub fn weighed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel::table(),
                self.weighed_with,
            ),
            conn,
        )
    }
    pub fn procedure_weighed_with<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.procedure_weighed_with,
            ),
            conn,
        )
    }
    pub fn procedure_fragment_source<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.procedure_fragment_source,
            ),
            conn,
        )
    }
    pub fn fragment_placed_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel::table(),
                self.fragment_placed_into,
            ),
            conn,
        )
    }
    pub fn procedure_fragment_placed_into<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::procedure_model_trackables::ProcedureModelTrackable::table(),
                self.procedure_fragment_placed_into,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableFractioningProcedureModelBuilder<
    ProcedureModel
        = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
> {
    pub(crate) kilograms: Option<f32>,
    pub(crate) tolerance_percentage: Option<f32>,
    pub(crate) weighed_with: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_weighed_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) procedure_fragment_source: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) fragment_placed_into: Option<::rosetta_uuid::Uuid>,
    pub(crate) procedure_fragment_placed_into: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    pub(crate) procedure_model: ProcedureModel,
}
impl<ProcedureModel> web_common_traits::database::ExtendableBuilder
for InsertableFractioningProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes,
    >,
{
    type Attributes = InsertableFractioningProcedureModelAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.procedure_model = self
            .procedure_model
            .extend_builder(other.procedure_model)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                ))
            })?;
        if let Some(kilograms) = other.kilograms {
            self = self.kilograms(kilograms)?;
        }
        if let Some(tolerance_percentage) = other.tolerance_percentage {
            self = self.tolerance_percentage(tolerance_percentage)?;
        }
        if let Some(weighed_with) = other.weighed_with {
            self = self.weighed_with(weighed_with)?;
        }
        self = self.procedure_weighed_with(other.procedure_weighed_with)?;
        self = self.procedure_fragment_source(other.procedure_fragment_source)?;
        if let Some(fragment_placed_into) = other.fragment_placed_into {
            self = self.fragment_placed_into(fragment_placed_into)?;
        }
        self = self
            .procedure_fragment_placed_into(other.procedure_fragment_placed_into)?;
        Ok(self)
    }
}
impl<ProcedureModel> web_common_traits::prelude::SetPrimaryKey
    for InsertableFractioningProcedureModelBuilder<ProcedureModel>
where
    ProcedureModel: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_model = self.procedure_model.set_primary_key(primary_key);
        self
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `fractioning_procedure_models.fragment_placed_into` column from table
    /// `fractioning_procedure_models`.
    pub fn fragment_placed_into(
        mut self,
        fragment_placed_into: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    > {
        self.fragment_placed_into = Some(fragment_placed_into);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `fractioning_procedure_models.kilograms` column
    /// from table `fractioning_procedure_models`.
    pub fn kilograms<Kilograms>(
        mut self,
        kilograms: Kilograms,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        Kilograms: TryInto<f32>,
        <Kilograms as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let kilograms =
            kilograms.try_into().map_err(|err: <Kilograms as TryInto<f32>>::Error| {
                Into::into(err)
                    .rename_field(InsertableFractioningProcedureModelAttributes::Kilograms)
            })?;
        pgrx_validation::must_be_strictly_positive_f32(kilograms)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::Kilograms,
                    )
            })?;
        self.kilograms = Some(kilograms);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `fractioning_procedure_models.procedure_fragment_placed_into` column
    /// from table `fractioning_procedure_models`.
    pub fn procedure_fragment_placed_into(
        mut self,
        mut procedure_fragment_placed_into: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFractioningProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.fragment_placed_into, procedure_fragment_placed_into.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureFragmentPlacedInto(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_fragment_placed_into.trackable_id {
            self.fragment_placed_into = Some(foreign);
        } else if let Some(local) = self.fragment_placed_into {
            procedure_fragment_placed_into = procedure_fragment_placed_into
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureFragmentPlacedInto(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_fragment_placed_into = self
            .procedure_fragment_placed_into
            .extend_builder(procedure_fragment_placed_into)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableFractioningProcedureModelAttributes::ProcedureFragmentPlacedInto(
                        attribute,
                    )
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `fractioning_procedure_models.procedure_fragment_source` column from
    /// table `fractioning_procedure_models`.
    pub fn procedure_fragment_source(
        mut self,
        procedure_fragment_source: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFractioningProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        self.procedure_fragment_source = self
            .procedure_fragment_source
            .extend_builder(procedure_fragment_source)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableFractioningProcedureModelAttributes::ProcedureFragmentSource(
                        attribute,
                    )
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `fractioning_procedure_models.procedure_weighed_with` column from table
    /// `fractioning_procedure_models`.
    pub fn procedure_weighed_with(
        mut self,
        mut procedure_weighed_with: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<
            InsertableFractioningProcedureModelAttributes,
        >,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::ExtendableBuilder<
            Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        >,
    {
        use web_common_traits::database::ExtendableBuilder;
        if let (Some(local), Some(foreign)) =
            (self.weighed_with, procedure_weighed_with.trackable_id)
        {
            if local != foreign {
                return Err(
                    web_common_traits::database::InsertError::BuilderError(
                        web_common_traits::prelude::BuilderError::UnexpectedAttribute(
                            crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureWeighedWith(
                                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes::TrackableId,
                            ),
                        ),
                    ),
                );
            }
        } else if let Some(foreign) = procedure_weighed_with.trackable_id {
            self.weighed_with = Some(foreign);
        } else if let Some(local) = self.weighed_with {
            procedure_weighed_with = procedure_weighed_with
                .trackable(local)
                .map_err(|e| {
                    e.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureWeighedWith(
                            attribute,
                        )
                    })
                })?;
        }
        self.procedure_weighed_with =
            self.procedure_weighed_with.extend_builder(procedure_weighed_with).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableFractioningProcedureModelAttributes::ProcedureWeighedWith(attribute)
                })
            })?;
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the
    /// `fractioning_procedure_models.tolerance_percentage` column from table
    /// `fractioning_procedure_models`.
    pub fn tolerance_percentage<TolerancePercentage>(
        mut self,
        tolerance_percentage: TolerancePercentage,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        TolerancePercentage: TryInto<f32>,
        <TolerancePercentage as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let tolerance_percentage = tolerance_percentage.try_into().map_err(
            |err: <TolerancePercentage as TryInto<f32>>::Error| {
                Into::into(err).rename_field(
                    InsertableFractioningProcedureModelAttributes::TolerancePercentage,
                )
            },
        )?;
        pgrx_validation::must_be_strictly_positive_f32(tolerance_percentage)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::TolerancePercentage,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(tolerance_percentage, 100f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::TolerancePercentage,
                            )
                    })
            })?;
        self.tolerance_percentage = Some(tolerance_percentage);
        Ok(self)
    }
}
impl<ProcedureModel>
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        ProcedureModel,
    >
{
    /// Sets the value of the `fractioning_procedure_models.weighed_with` column
    /// from table `fractioning_procedure_models`.
    pub fn weighed_with(
        mut self,
        weighed_with: ::rosetta_uuid::Uuid,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    > {
        self.weighed_with = Some(weighed_with);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at` column from table
    /// `fractioning_procedure_models`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.created_at(created_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_at`,
    /// `procedure_models.updated_at` columns from table
    /// `fractioning_procedure_models`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self
            .procedure_model
            .created_at_and_updated_at(created_at, updated_at)
            .map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableFractioningProcedureModelAttributes::Extension(
                        InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.created_by` column from table
    /// `fractioning_procedure_models`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.created_by(created_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.deprecated` column from table
    /// `fractioning_procedure_models`.
    pub fn deprecated<Deprecated>(
        mut self,
        deprecated: Deprecated,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        Deprecated: TryInto<bool>,
        <Deprecated as TryInto<bool>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.deprecated(deprecated).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.description` column from table
    /// `fractioning_procedure_models`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.description(description).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.icon` column from table
    /// `fractioning_procedure_models`.
    pub fn icon<Icon>(
        mut self,
        icon: Icon,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        Icon: TryInto<String>,
        <Icon as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.icon(icon).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name` column from table
    /// `fractioning_procedure_models`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.name(name).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.name`,
    /// `procedure_models.description` columns from table
    /// `fractioning_procedure_models`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model =
            self.procedure_model.name_and_description(name, description).map_err(|e| {
                e.into_field_name(|attribute| {
                    InsertableFractioningProcedureModelAttributes::Extension(
                        InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                            attribute,
                        ),
                    )
                })
            })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.photograph_id` column from table
    /// `fractioning_procedure_models`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.photograph(photograph_id).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_at` column from table
    /// `fractioning_procedure_models`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    >
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.procedure_model = self.procedure_model.updated_at(updated_at).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelBuilder,
    >
{
    /// Sets the value of the `procedure_models.updated_by` column from table
    /// `fractioning_procedure_models`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<
        Self,
        web_common_traits::database::InsertError<InsertableFractioningProcedureModelAttributes>,
    > {
        self.procedure_model = self.procedure_model.updated_by(updated_by).map_err(|e| {
            e.into_field_name(|attribute| {
                InsertableFractioningProcedureModelAttributes::Extension(
                    InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        attribute,
                    ),
                )
            })
        })?;
        Ok(self)
    }
}
impl<ProcedureModel, C> web_common_traits::database::TryInsertGeneric<C>
for InsertableFractioningProcedureModelBuilder<ProcedureModel>
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
        Error = web_common_traits::database::InsertError<
            InsertableFractioningProcedureModelAttributes,
        >,
    >,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
    >,
{
    type Attributes = InsertableFractioningProcedureModelAttributes;
    fn is_complete(&self) -> bool {
        self.procedure_model.is_complete() && self.kilograms.is_some()
            && self.tolerance_percentage.is_some() && self.weighed_with.is_some()
            && self.procedure_weighed_with.is_complete()
            && self.procedure_fragment_source.is_complete()
            && self.fragment_placed_into.is_some()
            && self.procedure_fragment_placed_into.is_complete()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        Self::PrimaryKey,
        web_common_traits::database::InsertError<Self::Attributes>,
    > {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
