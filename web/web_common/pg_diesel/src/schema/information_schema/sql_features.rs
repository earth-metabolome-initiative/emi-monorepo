//! `sql_features` view from `information_schema`.

diesel::table! {
    /// `information_schema.sql_features` — view containing information about
    /// SQL features supported by the database system. Shows which SQL standard
    /// features and sub-features are supported, verified, and any implementation comments.
    information_schema.sql_features (feature_id, sub_feature_id) {
        /// Identifier of the feature according to the SQL standard.
        feature_id -> Nullable<Text>,
        /// Descriptive name of the feature.
        feature_name -> Nullable<Text>,
        /// Identifier of the sub-feature (if applicable).
        sub_feature_id -> Nullable<Text>,
        /// Descriptive name of the sub-feature (if applicable).
        sub_feature_name -> Nullable<Text>,
        /// "YES" if the feature is supported, "NO" if not.
        is_supported -> Nullable<Text>,
        /// Name of the conformance test that verifies this feature.
        is_verified_by -> Nullable<Text>,
        /// Additional comments about the feature implementation.
        comments -> Nullable<Text>,
    }
}
