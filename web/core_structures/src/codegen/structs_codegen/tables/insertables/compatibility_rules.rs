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
            InsertableCompatibilityRuleAttributes::LeftTrackableId => {
                write!(f, "left_trackable_id")
            }
            InsertableCompatibilityRuleAttributes::RightTrackableId => {
                write!(f, "right_trackable_id")
            }
            InsertableCompatibilityRuleAttributes::Quantity => write!(f, "quantity"),
            InsertableCompatibilityRuleAttributes::CreatedBy => write!(f, "created_by"),
            InsertableCompatibilityRuleAttributes::CreatedAt => write!(f, "created_at"),
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
    left_trackable_id: ::rosetta_uuid::Uuid,
    right_trackable_id: ::rosetta_uuid::Uuid,
    quantity: Option<i16>,
    created_by: i32,
    created_at: ::rosetta_timestamp::TimestampUTC,
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
impl InsertableCompatibilityRuleBuilder {
    pub fn left_trackable_id<P>(
        mut self,
        left_trackable_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let left_trackable_id = left_trackable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err).rename_field(InsertableCompatibilityRuleAttributes::LeftTrackableId)
            },
        )?;
        if let Some(right_trackable_id) = self.right_trackable_id {
            pgrx_validation::must_be_distinct_uuid(left_trackable_id, right_trackable_id).map_err(
                |e| {
                    e.rename_fields(
                        InsertableCompatibilityRuleAttributes::LeftTrackableId,
                        InsertableCompatibilityRuleAttributes::RightTrackableId,
                    )
                },
            )?;
        }
        self.left_trackable_id = Some(left_trackable_id);
        Ok(self)
    }
    pub fn right_trackable_id<P>(
        mut self,
        right_trackable_id: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    where
        P: TryInto<::rosetta_uuid::Uuid>,
        <P as TryInto<::rosetta_uuid::Uuid>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let right_trackable_id = right_trackable_id.try_into().map_err(
            |err: <P as TryInto<::rosetta_uuid::Uuid>>::Error| {
                Into::into(err)
                    .rename_field(InsertableCompatibilityRuleAttributes::RightTrackableId)
            },
        )?;
        if let Some(left_trackable_id) = self.left_trackable_id {
            pgrx_validation::must_be_distinct_uuid(left_trackable_id, right_trackable_id).map_err(
                |e| {
                    e.rename_fields(
                        InsertableCompatibilityRuleAttributes::LeftTrackableId,
                        InsertableCompatibilityRuleAttributes::RightTrackableId,
                    )
                },
            )?;
        }
        self.right_trackable_id = Some(right_trackable_id);
        Ok(self)
    }
    pub fn quantity<P>(
        mut self,
        quantity: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    where
        P: TryInto<Option<i16>>,
        <P as TryInto<Option<i16>>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let quantity = quantity.try_into().map_err(|err: <P as TryInto<Option<i16>>>::Error| {
            Into::into(err).rename_field(InsertableCompatibilityRuleAttributes::Quantity)
        })?;
        if let Some(quantity) = quantity {
            pgrx_validation::must_be_strictly_positive_i16(quantity)
                .map_err(|e| e.rename_field(InsertableCompatibilityRuleAttributes::Quantity))?;
        }
        self.quantity = quantity;
        Ok(self)
    }
    pub fn created_by<P>(
        mut self,
        created_by: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    where
        P: TryInto<i32>,
        <P as TryInto<i32>>::Error: Into<validation_errors::SingleFieldError>,
    {
        let created_by = created_by.try_into().map_err(|err: <P as TryInto<i32>>::Error| {
            Into::into(err).rename_field(InsertableCompatibilityRuleAttributes::CreatedBy)
        })?;
        self.created_by = Some(created_by);
        Ok(self)
    }
    pub fn created_at<P>(
        mut self,
        created_at: P,
    ) -> Result<Self, web_common_traits::database::InsertError<InsertableCompatibilityRuleAttributes>>
    where
        P: TryInto<::rosetta_timestamp::TimestampUTC>,
        <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error:
            Into<validation_errors::SingleFieldError>,
    {
        let created_at = created_at.try_into().map_err(
            |err: <P as TryInto<::rosetta_timestamp::TimestampUTC>>::Error| {
                Into::into(err).rename_field(InsertableCompatibilityRuleAttributes::CreatedAt)
            },
        )?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl TryFrom<InsertableCompatibilityRuleBuilder> for InsertableCompatibilityRule {
    type Error = common_traits::prelude::BuilderError<InsertableCompatibilityRuleAttributes>;
    fn try_from(
        builder: InsertableCompatibilityRuleBuilder,
    ) -> Result<InsertableCompatibilityRule, Self::Error> {
        Ok(Self {
            left_trackable_id: builder.left_trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCompatibilityRuleAttributes::LeftTrackableId,
                ),
            )?,
            right_trackable_id: builder.right_trackable_id.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCompatibilityRuleAttributes::RightTrackableId,
                ),
            )?,
            quantity: builder.quantity,
            created_by: builder.created_by.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCompatibilityRuleAttributes::CreatedBy,
                ),
            )?,
            created_at: builder.created_at.ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    InsertableCompatibilityRuleAttributes::CreatedAt,
                ),
            )?,
        })
    }
}
