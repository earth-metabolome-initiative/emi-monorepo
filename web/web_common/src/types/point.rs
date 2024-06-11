//! Submodule providing the Point struct to be used for representing points in the database
//! within the context of postgis. This is the variant of the Point struct that is used in the
//! frontend, and as such does not have any diesel-specific code.

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, serde::Serialize, serde::Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    /// Spatial reference identifier
    pub srid: Option<u32>,
}

#[cfg(feature = "frontend")]
impl From<gluesql::core::data::Point> for Point {
    fn from(point: gluesql::core::data::Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
            srid: None
        }
    }
}

#[cfg(feature = "frontend")]
impl From<Point> for gluesql::core::data::Point {
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<Point> for (f64, f64) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

impl From<(f64, f64)> for Point {
    fn from(coords: (f64, f64)) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
            srid: None,
        }
    }
}

#[cfg(feature = "frontend")]
impl From<web_sys::Position> for Point {
    fn from(position: web_sys::Position) -> Self {
        let coords = position.coords();
        Self {
            x: coords.latitude(),
            y: coords.longitude(),
            srid: None,
        }
    }
}