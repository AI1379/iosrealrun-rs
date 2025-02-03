//
// Created by Renatus Madrigal on 02/03/2025
//

use geo::{Distance, Point};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Position {
    pub lat: f64,
    pub lng: f64,
}

fn sin(x: f64) -> f64 {
    return x.to_radians().sin();
}

fn cos(x: f64) -> f64 {
    return x.to_radians().cos();
}

fn transform_latitude(x: f64, y: f64) -> f64 {
    const PI: f64 = std::f64::consts::PI;
    let mut ret = -100.0 + 2.0 * x + 3.0 * y + 0.2 * y * y + 0.1 * x * y + 0.2 * x.abs().sqrt();
    ret += (20.0 * sin(6.0 * x * PI) + 20.0 * sin(2.0 * x * PI)) * 2.0 / 3.0;
    ret += (20.0 * sin(y * PI) + 40.0 * sin(y / 3.0 * PI)) * 2.0 / 3.0;
    ret += (160.0 * sin(y / 12.0 * PI) + 320.0 * sin(y * PI / 30.0)) * 2.0 / 3.0;
    return ret;
}

fn transform_longitude(x: f64, y: f64) -> f64 {
    const PI: f64 = std::f64::consts::PI;
    let mut ret = 300.0 + x + 2.0 * y + 0.1 * x * x + 0.1 * x * y + 0.1 * x.abs().sqrt();
    ret += (20.0 * sin(6.0 * x * PI) + 20.0 * sin(2.0 * x * PI)) * 2.0 / 3.0;
    ret += (20.0 * sin(x * PI) + 40.0 * sin(x / 3.0 * PI)) * 2.0 / 3.0;
    ret += (150.0 * sin(x / 12.0 * PI) + 300.0 * sin(x / 30.0 * PI)) * 2.0 / 3.0;
    return ret;
}

pub fn bd09_to_wgs84(pos: Position) -> Position {
    const PI: f64 = std::f64::consts::PI;
    const X_PI: f64 = PI * 3000.0 / 180.0;
    const A: f64 = 6378245.0;
    const EE: f64 = 0.00669342162296594323;

    let x = pos.lng - 0.0065;
    let y = pos.lat - 0.006;
    let z = (x * x + y * y).sqrt() - 0.00002 * sin(y * X_PI);
    let theta = y.atan2(x) - 0.000003 * cos(x * X_PI);

    let gcj_lng = z * theta.cos();
    let gcj_lat = z * theta.sin();

    let d_lng = transform_longitude(gcj_lng - 105.0, gcj_lat - 35.0);
    let d_lat = transform_latitude(gcj_lng - 105.0, gcj_lat - 35.0);

    return Position {
        lat: gcj_lat * 2.0 - gcj_lat - d_lat,
        lng: gcj_lng * 2.0 - gcj_lng - d_lng,
    };
}

pub fn geo_distance(pos1: Position, pos2: Position) -> f64 {
    return geo::Geodesic::distance(
        Point::new(pos1.lat, pos1.lng),
        Point::new(pos2.lat, pos2.lng),
    );
}
