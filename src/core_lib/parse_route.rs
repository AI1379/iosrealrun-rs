//
// Created by Renatus Madrigal on 02/03/2025
//

use super::coordinate::Position;
use serde_json::Value;

pub fn parse(json: Value) -> Vec<Position> {
    if !json.is_array() {
        panic!("Invalid JSON config");
    }
    return json
        .as_array()
        .unwrap()
        .iter()
        .map(|x| Position {
            latitude: x[0].as_f64().unwrap(),
            longitude: x[1].as_f64().unwrap(),
        })
        .collect();
}
