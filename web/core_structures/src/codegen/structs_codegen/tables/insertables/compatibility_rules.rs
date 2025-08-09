#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InsertableCompatibilityRuleAttributes {
    LeftTrackableId,
    RightTrackableId,
    Quantity,
    CreatedBy,
    CreatedAt,
}
impl core::fmt::Display for InsertableCompatibilityRuleAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::LeftTrackableId => write!(f, "left_trackable_id"),
            Self::RightTrackableId => write!(f, "right_trackable_id"),
            Self::Quantity => write!(f, "quantity"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::compatibility_rules::compatibility_rules
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCompatibilityRule {
    pub(crate) left_trackable_id: ::rosetta_uuid::Uuid,
    pub(crate) right_trackable_id: ::rosetta_uuid::Uuid,
    pub(crate) quantity: Option<i16>,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableCompatibilityRule {
    pub fn left_trackable<C: diesel::connection::LoadConnection>(
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
                self.left_trackable_id,
            ),
            conn,
        )
    }
    pub fn right_trackable<C: diesel::connection::LoadConnection>(
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
                self.right_trackable_id,
            ),
            conn,
        )
    }
    pub fn created_by<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::users::User,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::users::User: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::users::User as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::users::User as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::users::User,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::users::User::table(),
                self.created_by,
            ),
            conn,
        )
    }
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableCompatibilityRuleBuilder {
    pub(crate) left_trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) right_trackable_id: Option<::rosetta_uuid::Uuid>,
    pub(crate) quantity: Option<i16>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableCompatibilityRuleBuilder {
    fn default() -> Self {
        Self {
            left_trackable_id: Default::default(),
            right_trackable_id: Default::default(),
            quantity: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
impl web_common_traits::database::ExtendableBuilder for InsertableCompatibilityRuleBuilder {
    type Attributes = InsertableCompatibilityRuleAttributes;
    fn extend_builder(
        mut self,
        other: Self,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        match (other.left_trackable_id, other.right_trackable_id) {
            (Some(left_trackable_id), Some(right_trackable_id)) => {
                self =
                    self.left_trackable_and_right_trackable(left_trackable_id, right_trackable_id)?;
            }
            (None, Some(right_trackable_id)) => {
                self = self.right_trackable(right_trackable_id)?;
            }
            (Some(left_trackable_id), None) => {
                self = self.left_trackable(left_trackable_id)?;
            }
            (None, None) => {}
        }
        if let Some(quantity) = other.quantity {
            self = self.quantity(Some(quantity))?;
        }
        if let Some(created_by) = other.created_by {
            self = self.created_by(created_by)?;
        }
        if let Some(created_at) = other.created_at {
            self = self.created_at(created_at)?;
        }
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableCompatibilityRuleBuilder {
    type PrimaryKey = (::rosetta_uuid::Uuid, ::rosetta_uuid::Uuid);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleBuilder {
    /// Sets the value of the `compatibility_rules.created_at` column from table
    /// `compatibility_rules`.
    pub fn created_at<CreatedAt>(
        mut self,
        created_at: CreatedAt,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    where
        CreatedAt: TryInto<::rosetta_timestamp::TimestampUTC>,
        <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <CreatedAt as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableCompatibilityRuleAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleBuilder {
    /// Sets the value of the `compatibility_rules.created_by` column from table
    /// `compatibility_rules`.
    pub fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    {
        self.created_by = Some(created_by);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleBuilder {
    /// Sets the value of the `compatibility_rules.left_trackable_id` column
    /// from table `compatibility_rules`.
    pub fn left_trackable(
        mut self,
        left_trackable_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    {
        if let Some(right_trackable_id) = self.right_trackable_id {
            pgrx_validation::must_be_distinct_uuid(left_trackable_id, right_trackable_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleAttributes::LeftTrackableId,
                            crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleAttributes::RightTrackableId,
                        )
                })?;
        }
        self.left_trackable_id = Some(left_trackable_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleBuilder {
    /// Sets the value of the `compatibility_rules.left_trackable_id`,
    /// `compatibility_rules.right_trackable_id` columns from table
    /// `compatibility_rules`.
    pub fn left_trackable_and_right_trackable(
        mut self,
        left_trackable_id: ::rosetta_uuid::Uuid,
        right_trackable_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    {
        pgrx_validation::must_be_distinct_uuid(left_trackable_id, right_trackable_id)
            .map_err(|e| {
                e
                    .rename_fields(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleAttributes::LeftTrackableId,
                        crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleAttributes::RightTrackableId,
                    )
            })?;
        self.left_trackable_id = Some(left_trackable_id);
        self.right_trackable_id = Some(right_trackable_id);
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleBuilder {
    /// Sets the value of the `compatibility_rules.quantity` column from table
    /// `compatibility_rules`.
    pub fn quantity<Quantity>(
        mut self,
        quantity: Quantity,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    where
        Quantity: TryInto<Option<i16>>,
        <Quantity as TryInto<Option<i16>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let quantity =
            quantity.try_into().map_err(|err: <Quantity as TryInto<Option<i16>>>::Error| {
                Into::into(err).rename_field(InsertableCompatibilityRuleAttributes::Quantity)
            })?;
        if let Some(quantity) = quantity {
            pgrx_validation::must_be_strictly_positive_i16(quantity)
                .map_err(|e| {
                    e
                        .rename_field(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleAttributes::Quantity,
                        )
                })?;
        }
        self.quantity = quantity;
        Ok(self)
    }
}
impl crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleBuilder {
    /// Sets the value of the `compatibility_rules.right_trackable_id` column
    /// from table `compatibility_rules`.
    pub fn right_trackable(
        mut self,
        right_trackable_id: ::rosetta_uuid::Uuid,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    {
        if let Some(left_trackable_id) = self.left_trackable_id {
            pgrx_validation::must_be_distinct_uuid(left_trackable_id, right_trackable_id)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleAttributes::LeftTrackableId,
                            crate::codegen::structs_codegen::tables::insertables::InsertableCompatibilityRuleAttributes::RightTrackableId,
                        )
                })?;
        }
        self.right_trackable_id = Some(right_trackable_id);
        Ok(self)
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C> for InsertableCompatibilityRuleBuilder
where
    Self: web_common_traits::database::InsertableVariant<
            C,
            UserId = i32,
            Row = crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule,
            Error = web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>,
        >,
{
    type Attributes = InsertableCompatibilityRuleAttributes;
    fn is_complete(&self) -> bool {
        self.left_trackable_id.is_some()
            && self.right_trackable_id.is_some()
            && self.created_by.is_some()
            && self.created_at.is_some()
    }
    fn mint_primary_key(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>> {
        use diesel::Identifiable;
        use web_common_traits::database::InsertableVariant;
        let insertable: crate::codegen::structs_codegen::tables::compatibility_rules::CompatibilityRule = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
