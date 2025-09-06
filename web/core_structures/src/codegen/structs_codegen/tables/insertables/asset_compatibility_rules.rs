#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, core::fmt::Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssetCompatibilityRuleAttribute {
    LeftAssetModel,
    RightAssetModel,
    CreatedBy,
    CreatedAt,
}
impl core::str::FromStr for AssetCompatibilityRuleAttribute {
    type Err = web_common_traits::database::InsertError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LeftAssetModel" => Ok(Self::LeftAssetModel),
            "RightAssetModel" => Ok(Self::RightAssetModel),
            "CreatedBy" => Ok(Self::CreatedBy),
            "CreatedAt" => Ok(Self::CreatedAt),
            "left_asset_model" => Ok(Self::LeftAssetModel),
            "right_asset_model" => Ok(Self::RightAssetModel),
            "created_by" => Ok(Self::CreatedBy),
            "created_at" => Ok(Self::CreatedAt),
            _ => Err(web_common_traits::database::InsertError::UnknownAttribute(s.to_owned())),
        }
    }
}
impl core::fmt::Display for AssetCompatibilityRuleAttribute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::LeftAssetModel => write!(f, "left_asset_model"),
            Self::RightAssetModel => write!(f, "right_asset_model"),
            Self::CreatedBy => write!(f, "created_by"),
            Self::CreatedAt => write!(f, "created_at"),
        }
    }
}
#[cfg_attr(any(feature = "postgres", feature = "sqlite"), derive(diesel::Insertable))]
#[cfg_attr(
    any(feature = "postgres", feature = "sqlite"),
    diesel(
        table_name = crate::codegen::diesel_codegen::tables::asset_compatibility_rules::asset_compatibility_rules
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InsertableAssetCompatibilityRule {
    pub(crate) left_asset_model: i32,
    pub(crate) right_asset_model: i32,
    pub(crate) created_by: i32,
    pub(crate) created_at: ::rosetta_timestamp::TimestampUTC,
}
impl InsertableAssetCompatibilityRule {
    pub fn left_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.left_asset_model,
            ),
            conn,
        )
    }
    pub fn right_asset_model<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<
        crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::asset_models::AssetModel: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::asset_models::AssetModel as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::asset_models::AssetModel,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::asset_models::AssetModel::table(),
                self.right_asset_model,
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
pub struct InsertableAssetCompatibilityRuleBuilder {
    pub(crate) left_asset_model: Option<i32>,
    pub(crate) right_asset_model: Option<i32>,
    pub(crate) created_by: Option<i32>,
    pub(crate) created_at: Option<::rosetta_timestamp::TimestampUTC>,
}
impl Default for InsertableAssetCompatibilityRuleBuilder {
    fn default() -> Self {
        Self {
            left_asset_model: Default::default(),
            right_asset_model: Default::default(),
            created_by: Default::default(),
            created_at: Some(rosetta_timestamp::TimestampUTC::default()),
        }
    }
}
/// Trait defining setters for attributes of an instance of
/// `AssetCompatibilityRule` or descendant tables.
pub trait AssetCompatibilityRuleSettable: Sized {
    /// Attributes required to build the insertable.
    type Attributes;
    /// Sets the value of the
    /// `public.asset_compatibility_rules.left_asset_model` column.
    ///
    /// # Arguments
    /// * `left_asset_model`: The value to set for the
    ///   `public.asset_compatibility_rules.left_asset_model` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn left_asset_model(
        self,
        left_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the
    /// `public.asset_compatibility_rules.right_asset_model` column.
    ///
    /// # Arguments
    /// * `right_asset_model`: The value to set for the
    ///   `public.asset_compatibility_rules.right_asset_model` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn right_asset_model(
        self,
        right_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.asset_compatibility_rules.created_by`
    /// column.
    ///
    /// # Arguments
    /// * `created_by`: The value to set for the
    ///   `public.asset_compatibility_rules.created_by` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type `i32`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_by(
        self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>;
    /// Sets the value of the `public.asset_compatibility_rules.created_at`
    /// column.
    ///
    /// # Arguments
    /// * `created_at`: The value to set for the
    ///   `public.asset_compatibility_rules.created_at` column.
    ///
    /// # Implementation details
    /// This method accepts a reference to a generic value which can be
    /// converted to the required type for the column. This allows passing
    /// values of different types, as long as they can be converted to the
    /// required type using the `TryFrom` trait. The method, additionally,
    /// employs same-as and inferred same-as rules to ensure that the
    /// schema-defined ancestral tables and associated table values associated
    /// to the current column (if any) are also set appropriately.
    ///
    /// # Errors
    /// * If the provided value cannot be converted to the required type
    ///   `::rosetta_timestamp::TimestampUTC`.
    /// * If the provided value does not pass schema-defined validation.
    fn created_at<CA>(
        self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>;
}
impl AssetCompatibilityRuleSettable for InsertableAssetCompatibilityRuleBuilder {
    type Attributes =
        crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute;
    /// Sets the value of the
    /// `public.asset_compatibility_rules.left_asset_model` column.
    fn left_asset_model(
        mut self,
        left_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(right_asset_model) = self.right_asset_model {
            pgrx_validation::must_be_distinct_i32(left_asset_model, right_asset_model)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::LeftAssetModel,
                            crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::RightAssetModel,
                        )
                })?;
        }
        self.left_asset_model = Some(left_asset_model);
        Ok(self)
    }
    /// Sets the value of the
    /// `public.asset_compatibility_rules.right_asset_model` column.
    fn right_asset_model(
        mut self,
        right_asset_model: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        if let Some(left_asset_model) = self.left_asset_model {
            pgrx_validation::must_be_distinct_i32(left_asset_model, right_asset_model)
                .map_err(|e| {
                    e
                        .rename_fields(
                            crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::LeftAssetModel,
                            crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::RightAssetModel,
                        )
                })?;
        }
        self.right_asset_model = Some(right_asset_model);
        Ok(self)
    }
    /// Sets the value of the `public.asset_compatibility_rules.created_by`
    /// column.
    fn created_by(
        mut self,
        created_by: i32,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>> {
        self.created_by = Some(created_by);
        Ok(self)
    }
    /// Sets the value of the `public.asset_compatibility_rules.created_at`
    /// column.
    fn created_at<CA>(
        mut self,
        created_at: CA,
    ) -> Result<Self, web_common_traits::database::InsertError<Self::Attributes>>
    where
        CA: TryInto<::rosetta_timestamp::TimestampUTC>,
        validation_errors::SingleFieldError:
            From<<CA as TryInto<::rosetta_timestamp::TimestampUTC>>::Error>,
    {
        let created_at = created_at.try_into().map_err(|err| {
            validation_errors::SingleFieldError::from(err)
                .rename_field(AssetCompatibilityRuleAttribute::CreatedAt)
        })?;
        self.created_at = Some(created_at);
        Ok(self)
    }
}
impl web_common_traits::prelude::SetPrimaryKey for InsertableAssetCompatibilityRuleBuilder {
    type PrimaryKey = (i32, i32);
    fn set_primary_key(self, _primary_key: Self::PrimaryKey) -> Self {
        self
    }
}
impl<C> web_common_traits::database::TryInsertGeneric<C>
for InsertableAssetCompatibilityRuleBuilder
where
    Self: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule,
        Error = web_common_traits::database::InsertError<AssetCompatibilityRuleAttribute>,
    >,
{
    type Attributes = AssetCompatibilityRuleAttribute;
    fn is_complete(&self) -> bool {
        self.left_asset_model.is_some() && self.right_asset_model.is_some()
            && self.created_by.is_some() && self.created_at.is_some()
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
        let insertable: crate::codegen::structs_codegen::tables::asset_compatibility_rules::AssetCompatibilityRule = self
            .insert(user_id, conn)?;
        Ok(insertable.id())
    }
}
