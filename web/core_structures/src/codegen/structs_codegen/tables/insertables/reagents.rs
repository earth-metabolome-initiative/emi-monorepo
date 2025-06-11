#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableReagentAttributes {
    Id(crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes),
    Purity,
    CasCode,
    MolecularFormula,
}
impl From<crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes>
    for InsertableReagentAttributes
{
    fn from(
        extension: crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
    ) -> Self {
        Self::Id(extension)
    }
}
impl core::fmt::Display for InsertableReagentAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertableReagentAttributes::Id(id) => write!(f, "{}", id),
            InsertableReagentAttributes::Purity => write!(f, "purity"),
            InsertableReagentAttributes::CasCode => write!(f, "cas_code"),
            InsertableReagentAttributes::MolecularFormula => {
                write!(f, "molecular_formula")
            }
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
    id: ::rosetta_uuid::Uuid,
    purity: f32,
    cas_code: ::cas_codes::CAS,
    molecular_formula: ::molecular_formulas::MolecularFormula,
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
#[derive(Default)]
pub struct InsertableReagentBuilder {
    pub(crate) id: crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder,
    pub(crate) purity: Option<f32>,
    pub(crate) cas_code: Option<::cas_codes::CAS>,
    pub(crate) molecular_formula: Option<::molecular_formulas::MolecularFormula>,
}
impl InsertableReagentBuilder {
    pub fn purity<P>(
        mut self,
        purity: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<f32>,
        <P as TryInto<f32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let purity = purity.try_into().map_err(|err: <P as TryInto<f32>>::Error| {
            Into::into(err).rename_field(InsertableReagentAttributes::Purity)
        })?;
        pgrx_validation::must_be_strictly_positive_f32(purity)
            .map_err(|e| e.rename_field(InsertableReagentAttributes::Purity))
            .and_then(|_| {
                pgrx_validation::must_be_smaller_than_f32(purity, 100f32)
                    .map_err(|e| e.rename_field(InsertableReagentAttributes::Purity))
            })?;
        self.purity = Some(purity);
        Ok(self)
    }
    pub fn cas_code<P>(
        mut self,
        cas_code: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<::cas_codes::CAS>,
        <P as TryInto<::cas_codes::CAS>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let cas_code =
            cas_code.try_into().map_err(|err: <P as TryInto<::cas_codes::CAS>>::Error| {
                Into::into(err).rename_field(InsertableReagentAttributes::CasCode)
            })?;
        self.cas_code = Some(cas_code);
        Ok(self)
    }
    pub fn molecular_formula<P>(
        mut self,
        molecular_formula: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<::molecular_formulas::MolecularFormula>,
        <P as TryInto<::molecular_formulas::MolecularFormula>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let molecular_formula = molecular_formula.try_into().map_err(
            |err: <P as TryInto<::molecular_formulas::MolecularFormula>>::Error| {
                Into::into(err).rename_field(InsertableReagentAttributes::MolecularFormula)
            },
        )?;
        self.molecular_formula = Some(molecular_formula);
        Ok(self)
    }
    pub fn id<P>(
        mut self,
        id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.id(id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn name<P>(
        mut self,
        name: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.name(name).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn description<P>(
        mut self,
        description: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<Option<String>>,
        <P as TryInto<Option<String>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.description(description).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn photograph_id<P>(
        mut self,
        photograph_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.photograph_id(photograph_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn parent_id<P>(
        mut self,
        parent_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<Option<::rosetta_uuid::Uuid>>,
        <P as TryInto<Option<::rosetta_uuid::Uuid>>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.parent_id(parent_id).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_by(created_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.created_at(created_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_by<P>(
        mut self,
        updated_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_by(updated_by).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
    pub fn updated_at<P>(
        mut self,
        updated_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableReagentAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        self.id = self.id.updated_at(updated_at).map_err(|err| err.into_field_name())?;
        Ok(self)
    }
}
impl InsertableReagentBuilder {
    pub(crate) fn try_insert<C>(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<
        InsertableReagent,
        web_common_traits::database::InsertError<InsertableReagentAttributes>,
    >
    where
        crate::codegen::structs_codegen::tables::insertables::InsertableTrackableBuilder: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::trackables::Trackable,
            Error = web_common_traits::database::InsertError<
                crate::codegen::structs_codegen::tables::insertables::InsertableTrackableAttributes,
            >,
        >,
    {
        use diesel::associations::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let purity = self.purity.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
            InsertableReagentAttributes::Purity,
        ))?;
        let cas_code =
            self.cas_code.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableReagentAttributes::CasCode,
            ))?;
        let molecular_formula =
            self.molecular_formula.ok_or(common_traits::prelude::BuilderError::IncompleteBuild(
                InsertableReagentAttributes::MolecularFormula,
            ))?;
        let id = self.id.insert(user_id, conn).map_err(|err| err.into_field_name())?.id();
        Ok(InsertableReagent { id, purity, cas_code, molecular_formula })
    }
}
