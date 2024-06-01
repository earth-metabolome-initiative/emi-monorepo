use super::RowToBadge;
use web_common::database::NestedNameplate;

impl RowToBadge for NestedNameplate {
    fn badge_title(&self) -> String {
        format!("#{}", self.inner.barcode)
    }

    fn path(&self) -> Option<crate::router::AppRoute> {
        Some(<Self as crate::router::Viewable>::view_route(self))
    }

    fn font_awesome_icon(&self) -> Option<&str> {
        self.category.font_awesome_icon()
    }
}
