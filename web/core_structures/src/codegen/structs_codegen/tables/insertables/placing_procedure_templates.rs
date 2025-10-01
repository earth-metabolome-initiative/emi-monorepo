#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlacingProcedureTemplateExtensionAttribute {
    GeolocationProcedureTemplate(
        crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
    ),
}
impl core::fmt::Display for PlacingProcedureTemplateExtensionAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::GeolocationProcedureTemplate(e) => {
                write!(f, "placing_procedure_templates({e})")
            }
        }
    }
}
impl
    From<
        crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
    > for PlacingProcedureTemplateExtensionAttribute
{
    fn from(
        attribute: crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
    ) -> Self {
        Self::GeolocationProcedureTemplate(attribute)
    }
}
impl From<common_traits::builder::EmptyTuple> for PlacingProcedureTemplateExtensionAttribute {
    fn from(_attribute: common_traits::builder::EmptyTuple) -> Self {
        unreachable!("Some code generation error occurred to reach this point.")
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlacingProcedureTemplateAttribute {
    Extension(PlacingProcedureTemplateExtensionAttribute),
    ProcedureTemplate,
}
impl core::str::FromStr for PlacingProcedureTemplateAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ProcedureTemplate" => Ok(Self::ProcedureTemplate),
            "procedure_template" => Ok(Self::ProcedureTemplate),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl<T1> common_traits::builder::Attributed
for crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplateBuilder<
    T1,
> {
    type Attribute = PlacingProcedureTemplateAttribute;
}
impl web_common_traits::database::TableField for PlacingProcedureTemplateAttribute {}
impl web_common_traits::database::HasTableType for PlacingProcedureTemplateAttribute {
    type Table = crate::codegen::tables::table_names::TableName;
}
impl
    web_common_traits::database::FromExtension<
        crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
    > for PlacingProcedureTemplateAttribute
{
    fn from_extension(
        attribute: crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
    ) -> Self {
        PlacingProcedureTemplateAttribute::Extension(From::from(attribute))
    }
}
impl web_common_traits::database::FromExtension<common_traits::builder::EmptyTuple>
    for PlacingProcedureTemplateAttribute
{
    fn from_extension(attribute: common_traits::builder::EmptyTuple) -> Self {
        PlacingProcedureTemplateAttribute::Extension(From::from(attribute))
    }
}
impl core::fmt::Display for PlacingProcedureTemplateAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::ProcedureTemplate => {
                write!(f, "placing_procedure_templates.procedure_template")
            }
        }
    }
}
#[derive(Debug)]
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::placing_procedure_templates::placing_procedure_templates
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertablePlacingProcedureTemplate {
    pub(crate) procedure_template: i32,
}
impl InsertablePlacingProcedureTemplate {}
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Builder for creating and inserting a new
/// [`PlacingProcedureTemplate`](crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate).
///
///
/// # Implementation details
/// While this builder implements several methods, a reasonably complete
/// **basic** usage example (*which may not apply to your own specific use case,
/// please adapt accordingly*) is as follows:
///
/// ```rust,ignore
/// use core_structures::PlacingProcedureTemplate;
/// use core_structures::tables::insertables::GeolocationProcedureTemplateSettable;
/// use core_structures::tables::insertables::ProcedureTemplateSettable;
/// use web_common_traits::database::Insertable;
/// use web_common_traits::database::InsertableVariant;
///
/// let placing_procedure_template = PlacingProcedureTemplate::new()
///    // Set mandatory fields
///    .procedure_template_geolocated_asset_model(procedure_template_geolocated_asset_model)?
///    .procedure_template_geolocated_with_model(procedure_template_geolocated_with_model)?
///    .created_by(created_by)?
///    .description(description)?
///    .name(name)?
///    // Note: `updated_by` is automatically set by the `created by` column.
///    .updated_by(updated_by)?
///    // Optionally set fields with default values
///    .created_at(created_at)?
///    .deprecated(deprecated)?
///    .updated_at(updated_at)?
///    // Finally, insert the new record in the database
///    .insert(user.id, conn)?;
/// ```
pub struct InsertablePlacingProcedureTemplateBuilder<
    GeolocationProcedureTemplate
        = crate::codegen::structs_codegen::tables::insertables::InsertableGeolocationProcedureTemplateBuilder<
            crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateBuilder,
        >,
> {
    pub(crate) procedure_template: GeolocationProcedureTemplate,
}
impl<GeolocationProcedureTemplate> diesel::associations::HasTable
    for InsertablePlacingProcedureTemplateBuilder<GeolocationProcedureTemplate>
{
    type Table = crate::codegen::diesel_codegen::tables::placing_procedure_templates::placing_procedure_templates::table;
    fn table() -> Self::Table {
        crate::codegen::diesel_codegen::tables::placing_procedure_templates::placing_procedure_templates::table
    }
}
impl From<InsertablePlacingProcedureTemplateBuilder>
    for web_common_traits::database::IdOrBuilder<i32, InsertablePlacingProcedureTemplateBuilder>
{
    fn from(builder: InsertablePlacingProcedureTemplateBuilder) -> Self {
        Self::Builder(builder)
    }
}
impl<GeolocationProcedureTemplate> common_traits::builder::IsCompleteBuilder
for crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureTemplateBuilder<
    GeolocationProcedureTemplate,
>
where
    GeolocationProcedureTemplate: common_traits::builder::IsCompleteBuilder,
{
    fn is_complete(&self) -> bool {
        self.procedure_template.is_complete()
    }
}
/// Trait defining setters for attributes of an instance of
/// `PlacingProcedureTemplate` or descendant tables.
pub trait PlacingProcedureTemplateSettable: Sized {
    /// Error type returned when setting attributes.
    type Error;
}
impl<GeolocationProcedureTemplate> PlacingProcedureTemplateSettable
for InsertablePlacingProcedureTemplateBuilder<GeolocationProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::PlacingProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
}
impl<
    GeolocationProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateSettable
for InsertablePlacingProcedureTemplateBuilder<GeolocationProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::PlacingProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.geolocation_procedure_templates.geolocated_with_model` column.
    fn geolocated_with_model<GWM>(
        mut self,
        geolocated_with_model: GWM,
    ) -> Result<Self, Self::Error>
    where
        GWM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateSettable>::geolocated_with_model(
                self.procedure_template,
                geolocated_with_model,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.geolocation_procedure_templates.procedure_template_geolocated_with_model` column.
    fn procedure_template_geolocated_with_model<PTGWM>(
        mut self,
        procedure_template_geolocated_with_model: PTGWM,
    ) -> Result<Self, Self::Error>
    where
        PTGWM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateSettable>::procedure_template_geolocated_with_model(
                self.procedure_template,
                procedure_template_geolocated_with_model,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.geolocation_procedure_templates.geolocated_asset_model` column.
    fn geolocated_asset_model<GAM>(
        mut self,
        geolocated_asset_model: GAM,
    ) -> Result<Self, Self::Error>
    where
        GAM: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateSettable>::geolocated_asset_model(
                self.procedure_template,
                geolocated_asset_model,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.geolocation_procedure_templates.procedure_template_geolocated_asset_model` column.
    fn procedure_template_geolocated_asset_model<PTGAM>(
        mut self,
        procedure_template_geolocated_asset_model: PTGAM,
    ) -> Result<Self, Self::Error>
    where
        PTGAM: Into<
            web_common_traits::database::IdOrBuilder<
                i32,
                crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTemplateAssetModelBuilder,
            >,
        >,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateSettable>::procedure_template_geolocated_asset_model(
                self.procedure_template,
                procedure_template_geolocated_asset_model,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<
    GeolocationProcedureTemplate: crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable<
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::GeolocationProcedureTemplateAttribute,
            >,
        >,
> crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable
for InsertablePlacingProcedureTemplateBuilder<GeolocationProcedureTemplate>
where
    Self: common_traits::builder::Attributed<
        Attribute = crate::codegen::structs_codegen::tables::insertables::PlacingProcedureTemplateAttribute,
    >,
{
    type Error = web_common_traits::database::InsertError<
        <Self as common_traits::builder::Attributed>::Attribute,
    >;
    #[inline]
    ///Sets the value of the `public.procedure_templates.name` column.
    fn name<N>(mut self, name: N) -> Result<Self, Self::Error>
    where
        N: TryInto<String>,
        validation_errors::SingleFieldError: From<<N as TryInto<String>>::Error>,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::name(
                self.procedure_template,
                name,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.description` column.
    fn description<D>(mut self, description: D) -> Result<Self, Self::Error>
    where
        D: TryInto<String>,
        validation_errors::SingleFieldError: From<<D as TryInto<String>>::Error>,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::description(
                self.procedure_template,
                description,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.created_by` column.
    fn created_by<CB>(mut self, created_by: CB) -> Result<Self, Self::Error>
    where
        CB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_by(
                self.procedure_template,
                created_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.created_at` column.
    fn created_at<CA>(mut self, created_at: CA) -> Result<Self, Self::Error>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::created_at(
                self.procedure_template,
                created_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.updated_by` column.
    fn updated_by<UB>(mut self, updated_by: UB) -> Result<Self, Self::Error>
    where
        UB: web_common_traits::database::PrimaryKeyLike<PrimaryKey = i32>,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_by(
                self.procedure_template,
                updated_by,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.updated_at` column.
    fn updated_at<UA>(mut self, updated_at: UA) -> Result<Self, Self::Error>
    where
        UA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError: From<
            <UA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error,
        >,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::updated_at(
                self.procedure_template,
                updated_at,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
    #[inline]
    ///Sets the value of the `public.procedure_templates.deprecated` column.
    fn deprecated<D>(mut self, deprecated: D) -> Result<Self, Self::Error>
    where
        D: TryInto<bool>,
        validation_errors::SingleFieldError: From<<D as TryInto<bool>>::Error>,
    {
        self.procedure_template = <GeolocationProcedureTemplate as crate::codegen::structs_codegen::tables::insertables::ProcedureTemplateSettable>::deprecated(
                self.procedure_template,
                deprecated,
            )
            .map_err(|e| {
                e
                    .into_field_name(|attribute| <Self as common_traits::builder::Attributed>::Attribute::Extension(
                        attribute.into(),
                    ))
            })?;
        Ok(self)
    }
}
impl<GeolocationProcedureTemplate> web_common_traits::database::MostConcreteTable
    for InsertablePlacingProcedureTemplateBuilder<GeolocationProcedureTemplate>
where
    GeolocationProcedureTemplate: web_common_traits::database::MostConcreteTable,
{
    fn set_most_concrete_table(&mut self, table_name: &str) {
        self.procedure_template.set_most_concrete_table(table_name);
    }
}
impl<GeolocationProcedureTemplate> web_common_traits::prelude::SetPrimaryKey
    for InsertablePlacingProcedureTemplateBuilder<GeolocationProcedureTemplate>
where
    GeolocationProcedureTemplate: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = i32>,
{
    type PrimaryKey = i32;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.procedure_template = self.procedure_template.set_primary_key(primary_key);
        self
    }
}
impl<GeolocationProcedureTemplate, C> web_common_traits::database::TryInsertGeneric<C>
for InsertablePlacingProcedureTemplateBuilder<GeolocationProcedureTemplate>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
            C,
            Row = crate::codegen::structs_codegen::tables::placing_procedure_templates::PlacingProcedureTemplate,
            Error = web_common_traits::database::InsertError<
                PlacingProcedureTemplateAttribute,
            >,
        > + web_common_traits::database::SetPrimaryKey<PrimaryKey = i32>
        + common_traits::builder::IsCompleteBuilder,
{
    type Error = web_common_traits::database::InsertError<
        PlacingProcedureTemplateAttribute,
    >;
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, Self::Error> {
        use diesel::Identifiable;
        use web_common_traits::database::DispatchableInsertableVariant;
        Ok(self.insert(user_id, conn)?.id())
    }
}
