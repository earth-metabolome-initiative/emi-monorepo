use super::RowToBadge;
use web_common::database::NestedUnit;

impl RowToBadge for NestedUnit {
    fn badge_title(&self) -> String {
        "UNIT".to_owned()
    }
}
