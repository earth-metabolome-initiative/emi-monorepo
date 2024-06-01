use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::Unit;

impl RowToBadge for Unit {
    fn badge_title(&self) -> String {
        "UNIT".to_owned()
    }
}
