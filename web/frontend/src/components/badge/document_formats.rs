use super::{Badge, BadgeSize, RowToBadge};
use web_common::database::NestedDocumentFormat;

impl RowToBadge for NestedDocumentFormat {
    fn badge_title(&self) -> String {
        format!("{} ({})", self.inner.extension, self.inner.mime_type)
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.icon.font_awesome_icon()
    }
}
