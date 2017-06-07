use xmltree::Element;

use Point;
use Error;
use try_from::TryFrom;

#[derive(Debug)]
pub enum Geometry {
    Polygon(Vec<Point>),
}

impl<'a> TryFrom<&'a Element> for Geometry {
    type Err = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Err> {
        let polygon = element.get_child("POLYGON").unwrap();
        let text = polygon.text.as_ref().unwrap();
        let points = text.split(",").map(|s| s.parse().unwrap());

        Ok(Geometry::Polygon(points.collect()))
    }
}