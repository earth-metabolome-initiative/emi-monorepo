//! Submodule providing the Point struct to be used for representing points in the database
//! within the context of postgis.
use super::Convert;

impl Convert<web_common::types::Point> for postgis_diesel::types::Point {
    fn convert(self) -> web_common::types::Point {
        web_common::types::Point {
            x: self.x,
            y: self.y,
            srid: self.srid,
        }
    }
}

impl Convert<postgis_diesel::types::Point> for web_common::types::Point {
    fn convert(self) -> postgis_diesel::types::Point {
        postgis_diesel::types::Point::new(self.x, self.y, self.srid)
    }
}
