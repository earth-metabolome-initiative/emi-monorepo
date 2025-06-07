#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod as_ref;
pub mod diesel_impls;
mod display;
pub mod errors;
mod try_from;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum_macros::EnumIter)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(
	feature = "diesel",
	diesel(sql_type = crate::diesel_impls::NameplateCategory)
)]
/// Enum representing the tool categories.
pub enum NameplateCategory {
    /// An digital nameplate that is used to digitally label specimens,
    /// but has no physical counterpart.
    Digital,
    /// An Aluminium nameplate with a brushed finish and adhesive backing.
    /// It is used for labeling specimens such as trees.
    /// Placed at human height and nailed to the trunk.
    Permanent,
    /// A plastified adhesive nameplate with a white background and black
    /// QR code for the specimen or container.
    SemiPermanent,
}

impl NameplateCategory {
    #[must_use]
    /// Returns the name of the nameplate category.
    pub fn name(&self) -> &'static str {
        match self {
            NameplateCategory::Digital => "Digital",
            NameplateCategory::Permanent => "Permanent",
            NameplateCategory::SemiPermanent => "Semi-Permanent",
        }
    }

    #[must_use]
    /// Returns the description of the nameplate category.
    pub fn description(&self) -> &'static str {
        match self {
            NameplateCategory::Digital => {
                "A digital nameplate that is used to digitally label specimens, \
                but has no physical counterpart."
            }
            NameplateCategory::Permanent => {
                "An Aluminium nameplate with a brushed finish and adhesive backing. \
                It is used for labeling specimens such as trees. Placed at human height \
                and nailed to the trunk."
            }
            NameplateCategory::SemiPermanent => {
                "A plastified adhesive nameplate with a white background and black \
                QR code for the specimen or container."
            }
        }
    }

    #[must_use]
    /// Returns the icon of the nameplate category.
    pub fn icon(&self) -> &'static str {
        match self {
            NameplateCategory::Digital => "computer",
            NameplateCategory::Permanent => "tag",
            NameplateCategory::SemiPermanent => "qrcode",
        }
    }
}
