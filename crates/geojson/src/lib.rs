#[macro_use]
extern crate serde_derive;
extern crate serde;

use serde_json as json;
use std::{fmt::Display, str::FromStr};

pub type Coordinate = (f64, f64);

pub trait LatLng {
    fn lng(&self) -> f64;
    fn lat(&self) -> f64;
}

impl LatLng for Coordinate {
    fn lng(&self) -> f64 {
        self.0
    }
    fn lat(&self) -> f64 {
        self.1
    }
}

pub type Properties = json::Map<String, json::Value>;

pub trait Value {
    fn value<T>(&self, name: &str) -> Option<T>
    where
        for<'de> T: serde::Deserialize<'de>;
}

impl Value for Properties {
    fn value<T>(&self, name: &str) -> Option<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        match self.get(name) {
            Some(value) => json::from_value::<T>(value.clone()).ok(),
            _ => None,
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Point {
    pub coordinates: Coordinate,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct LineString {
    pub coordinates: Vec<Coordinate>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct Polygon {
    pub coordinates: Vec<Vec<Coordinate>>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct MultiPoint {
    pub coordinates: Vec<Coordinate>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct MultiLineString {
    pub coordinates: Vec<Vec<Coordinate>>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct MultiPolygon {
    pub coordinates: Vec<Vec<Vec<Coordinate>>>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct GeometryCollection {
    pub geometries: Vec<Geometry>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Geometry {
    Point(Point),
    LineString(LineString),
    Polygon(Polygon),
    MultiPoint(MultiPoint),
    MultiLineString(MultiLineString),
    MultiPolygon(MultiPolygon),
    GeometryCollection(GeometryCollection),
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Feature {
    pub id: Option<u64>,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum GeoJson {
    Point(Point),
    LineString(LineString),
    Polygon(Polygon),
    MultiPoint(MultiPoint),
    MultiLineString(MultiLineString),
    MultiPolygon(MultiPolygon),
    GeometryCollection(GeometryCollection),
    Feature(Feature),
    FeatureCollection(FeatureCollection),
}

impl FromStr for GeoJson {
    type Err = json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        json::from_str(s)
    }
}

#[cfg(test)]
mod tests {

    use json::Number;

    use super::*;

    #[test]
    fn test_geometry_polygon_deserialize() {
        let json_str = r##"{
         "type": "Polygon",
         "coordinates": [
             [
                 [100.0, 0.0],
                 [101.0, 0.0],
                 [101.0, 1.0],
                 [100.0, 1.0],
                 [100.0, 0.0]
             ],
             [
                 [100.8, 0.8],
                 [100.8, 0.2],
                 [100.2, 0.2],
                 [100.2, 0.8],
                 [100.8, 0.8]
             ]
         ]
     }"##;

        let actual = json_str.parse::<GeoJson>().unwrap();
        let expected = GeoJson::Polygon(Polygon {
            coordinates: vec![
                vec![
                    (100.0, 0.0),
                    (101.0, 0.0),
                    (101.0, 1.0),
                    (100.0, 1.0),
                    (100.0, 0.0),
                ],
                vec![
                    (100.8, 0.8),
                    (100.8, 0.2),
                    (100.2, 0.2),
                    (100.2, 0.8),
                    (100.8, 0.8),
                ],
            ],
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_feature_polygon_deserialize() {
        let json_str = r##"{
            "id": 588419,
            "geometry": {
            "coordinates": [
                [
                    [77.35, 12.75],
                    [77.35, 13.23],
                    [77.85, 13.23],
                    [77.85, 12.75],
                    [77.35, 12.75]
                ]
            ],
            "type": "Polygon"
        },
            "properties": {
            "FID" : 588419
        },
            "type": "Feature"
        }"##;
        let actual: GeoJson = json_str.parse::<GeoJson>().unwrap();

        if let GeoJson::Feature(Feature {
            id: _,
            ref properties,
            geometry: _,
        }) = actual
        {
            let fid = properties.value::<i32>("FID");
            assert_eq!(Some(588419), fid);
        }

        let mut prop = serde_json::Map::new();
        prop.insert("FID".to_owned(), json::Value::Number(Number::from(588419)));

        let expected = GeoJson::Feature(Feature {
            id: Some(588419),
            geometry: Geometry::Polygon(Polygon {
                coordinates: vec![vec![
                    (77.35, 12.75),
                    (77.35, 13.23),
                    (77.85, 13.23),
                    (77.85, 12.75),
                    (77.35, 12.75),
                ]],
            }),
            properties: prop,
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_feature_linestring_deserialize() {
        let json_str = r##"
        {
            "id": 100001035,
            "geometry": {
                "coordinates": [
                    [77.52420806884766, 12.987045288085938],
                    [77.5242919921875, 12.98731803894043],
                    [77.52436065673828, 12.987621307373049],
                    [77.52440643310547, 12.987802505493164],
                    [77.52530670166016, 12.988258361816406]
                ],
                "type": "LineString"
            },
            "properties": {
                "startNodeId": 1548580548,
                "endNodeId": 1548580888,
                "highway": "residential"
            },
            "type": "Feature"
        }
        "##;
        let actual = json_str.parse::<GeoJson>().unwrap();

        let prop = serde_json::Map::from_iter([
            (
                "startNodeId".to_owned(),
                json::Value::Number(Number::from(1548580548)),
            ),
            (
                "endNodeId".to_owned(),
                json::Value::Number(Number::from(1548580888)),
            ),
            (
                "highway".to_owned(),
                json::Value::String("residential".to_owned()),
            ),
        ]);

        let expected = GeoJson::Feature(Feature {
            id: Some(100001035),
            geometry: Geometry::LineString(LineString {
                coordinates: vec![
                    (77.52420806884766, 12.987045288085938),
                    (77.5242919921875, 12.98731803894043),
                    (77.52436065673828, 12.987621307373049),
                    (77.52440643310547, 12.987802505493164),
                    (77.52530670166016, 12.988258361816406),
                ],
            }),
            properties: prop,
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_feature_point_deserialize() {
        let json_str = r##"
        {
            "id": 1,
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [102.0, 0.5]
            },
            "properties": {
                "prop0": "value0"
            }
        }"##;

        let prop = serde_json::Map::from_iter([(
            "prop0".to_owned(),
            json::Value::String("value0".to_owned()),
        )]);

        let actual = json_str.parse::<GeoJson>().unwrap();

        let expected = GeoJson::Feature(Feature {
            id: Some(1),
            geometry: Geometry::Point(Point {
                coordinates: (102.0, 0.5),
            }),
            properties: prop,
        });

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_invalid_geojson() {
        let json_str = r##"
        {
            "type": "jadlf"
        }"##;
        assert_eq!(true, json_str.parse::<GeoJson>().is_err());
    }

    #[test]
    fn test_properties_value_get() {
        let map_json = serde_json::Map::from_iter([(
            "hello".to_string(),
            json::Value::Object(json::Map::from_iter([
                ("name".to_string(), json::Value::String("John".to_string())),
                (
                    "surname".to_string(),
                    json::Value::String("Doe".to_string()),
                ),
            ])),
        )]);
        let prop: Properties = map_json;

        let actual = prop.value::<json::Value>("hello");
        assert_eq!(
            Some(json::Value::Object(json::Map::from_iter([
                ("name".to_string(), json::Value::String("John".to_string())),
                (
                    "surname".to_string(),
                    json::Value::String("Doe".to_string())
                )
            ]))),
            actual
        );
    }
}
