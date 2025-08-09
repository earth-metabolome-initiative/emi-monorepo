#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableReagentExtensionAttributes {
    Trackable(crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes),
}
impl core::fmt::Display for InsertableReagentExtensionAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Trackable(e) => write!(f, "{e}"),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableReagentAttributes {
    Extension(InsertableReagentExtensionAttributes),
    Id,
    Purity,
    CasCode,
    MolecularFormula,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes>
    for InsertableReagentAttributes
{
    fn from(
        trackables: crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    ) -> Self {
        Self::Extension(InsertableReagentExtensionAttributes::Trackable(trackables))
    }
}
impl core::fmt::Display for InsertableReagentAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Extension(e) => write!(f, "{e}"),
            Self::Id => write!(f, "id"),
            Self::Purity => write!(f, "purity"),
            Self::CasCode => write!(f, "cas_code"),
            Self::MolecularFormula => write!(f, "molecular_formula"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(table_name = crate::codegen::diesel_codegen::tables::reagents::reagents)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableReagent {
    pub(crate) id: ::rosetta_uuid::Uuid,
    pub(crate) purity: f32,
    pub(crate) cas_code: ::cas_codes::CAS,
    pub(crate) molecular_formula: ::molecular_formulas::MolecularFormula,
}
impl InsertableReagent {
    pub fn id<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::trackables::Trackable,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::trackables::Trackable: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::trackables::Trackable as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::trackables::Trackable,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::trackables::Trackable::table(),
                self.id,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableReagentBuilder<
    Trackable = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
> {
    pub(crate) purity: Option<f32>,
    pub(crate) cas_code: Option<::cas_codes::CAS>,
    pub(crate) molecular_formula: Option<::molecular_formulas::MolecularFormula>,
    pub(crate) id: Trackable,
}
impl<Trackable> web_common_traits::database::ExtendableBuilder
for InsertableReagentBuilder<Trackable>
where
    Trackable: web_common_traits::database::ExtendableBuilder<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    >,
{
    type Attributes = InsertableReagentAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.id = self
            .id
            .extend_builder(other.id)
            .map_err(|err| {
                err.into_field_name(|attribute| InsertableReagentAttributes::Extension(
                    InsertableReagentExtensionAttributes::Trackable(attribute),
                ))
            })?;
        if let Some(purity) = other.purity {
            self = self.purity(purity)?;
        }
        if let Some(cas_code) = other.cas_code {
            self = self.cas_code(cas_code)?;
        }
        if let Some(molecular_formula) = other.molecular_formula {
            self = self.molecular_formula(molecular_formula)?;
        }
        Ok(self)
    }
}
impl<Trackable> web_common_traits::prelude::SetPrimaryKey for InsertableReagentBuilder<Trackable>
where
    Trackable: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type PrimaryKey = ::rosetta_uuid::Uuid;
    fn set_primary_key(mut self, primary_key: Self::PrimaryKey) -> Self {
        self.id = self.id.set_primary_key(primary_key);
        self
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<Trackable>
{
    /// Sets the value of the `reagents.cas_code` column from table `reagents`.
    pub fn cas_code<CasCode>(
        mut self,
        cas_code: CasCode,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        CasCode: TryInto<::cas_codes::CAS>,
        <CasCode as TryInto<::cas_codes::CAS>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let cas_code =
            cas_code.try_into().map_err(|err: <CasCode as TryInto<::cas_codes::CAS>>::Error| {
                Into::into(err).rename_field(InsertableReagentAttributes::CasCode)
            })?;
        self.cas_code = Some(cas_code);
        Ok(self)
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<Trackable>
{
    /// Sets the value of the `reagents.molecular_formula` column from table
    /// `reagents`.
    pub fn molecular_formula<MolecularFormula>(
        mut self,
        molecular_formula: MolecularFormula,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        MolecularFormula: TryInto<::molecular_formulas::MolecularFormula>,
        <MolecularFormula as TryInto<::molecular_formulas::MolecularFormula>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let molecular_formula = molecular_formula.try_into().map_err(
            |err: <MolecularFormula as TryInto<::molecular_formulas::MolecularFormula>>::Error| {
                Into::into(err).rename_field(InsertableReagentAttributes::MolecularFormula)
            },
        )?;
        self.molecular_formula = Some(molecular_formula);
        Ok(self)
    }
}
impl<Trackable>
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<Trackable>
{
    /// Sets the value of the `reagents.purity` column from table `reagents`.
    pub fn purity<Purity>(
        mut self,
        purity: Purity,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        Purity: TryInto<f32>,
        <Purity as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let purity = purity.try_into().map_err(|err: <Purity as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableReagentAttributes::Purity)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(purity)
            .map_err(|e| {
                e
                    .rename_field(
                        crate::codegen::structs_codegen::tables::insertables::InsertableReagentAttributes::Purity,
                    )
            })
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(purity, 100f32)
                    .map_err(|e| {
                        e
                            .rename_field(
                                crate::codegen::structs_codegen::tables::insertables::InsertableReagentAttributes::Purity,
                            )
                    })
            })?;
        self.purity = Some(purity);
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_at` column from table
    /// `reagents`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_at`, `trackables.updated_at`
    /// columns from table `reagents`.
    pub fn created_at_and_updated_at<CreatedAt, UpdatedAt>(
        mut self,
        created_at: CreatedAt,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .created_at_and_updated_at(created_at, updated_at)
            .map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.created_by` column from table
    /// `reagents`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>> {
        self.id = self.id.created_by(created_by).map_err(|e| e.into_field_name(From::from))?;
        self = self.updated_by(created_by)?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.description` column from table
    /// `reagents`.
    pub fn description<Description>(
        mut self,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        Description: TryInto<Option<String>>,
        <Description as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.id` column from table `reagents`.
    pub fn id<Id>(
        mut self,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.name` column from table `reagents`.
    pub fn name<Name>(
        mut self,
        name: Name,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        Name: TryInto<Option<String>>,
        <Name as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.name`, `trackables.description`
    /// columns from table `reagents`.
    pub fn name_and_description<Name, Description>(
        mut self,
        name: Name,
        description: Description,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        Name: TryInto<String>,
        <Name as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
        Description: TryInto<String>,
        <Description as TryInto<String>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self
            .id
            .name_and_description(name, description)
            .map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.parent_id` column from table
    /// `reagents`.
    pub fn parent(
        mut self,
        parent_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>> {
        self.id = self.id.parent(parent_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.parent_id`, `trackables.id` columns
    /// from table `reagents`.
    pub fn parent_and_id<Id>(
        mut self,
        parent_id: ::rosetta_uuid::Uuid,
        id: Id,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        Id: TryInto<::rosetta_uuid::Uuid>,
        <Id as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id =
            self.id.parent_and_id(parent_id, id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.photograph_id` column from table
    /// `reagents`.
    pub fn photograph(
        mut self,
        photograph_id: Option<::rosetta_uuid::Uuid>,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>> {
        self.id = self.id.photograph(photograph_id).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_at` column from table
    /// `reagents`.
    pub fn updated_at<UpdatedAt>(
        mut self,
        updated_at: UpdatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        UpdatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <UpdatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl
    crate::codegen::structs_codegen::tables::insertables::InsertableReagentBuilder<
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    >
{
    /// Sets the value of the `trackables.updated_by` column from table
    /// `reagents`.
    pub fn updated_by(
        mut self,
        updated_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>> {
        self.id = self.id.updated_by(updated_by).map_err(|e| e.into_field_name(From::from))?;
        Ok(self)
    }
}
impl<Trackable, C> web_common_traits::database::TryInsertGeneric<C>
    for InsertableReagentBuilder<Trackable>
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::reagents::Reagent,
            Error = web_common_traits::database::InsertError<InsertableReagentAttributes>,
        >,
    Trackable: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = ::rosetta_uuid::Uuid>,
{
    type Attributes = InsertableReagentAttributes;
    fn is_complete(&self) -> bool {
        self.id.is_complete()
            && self.purity.is_some()
            && self.cas_code.is_some()
            && self.molecular_formula.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::reagents::Reagent =
            self.insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
