//
// Created by Renatus Madrigal on 02/03/2025
//

use super::coordinate::{geo_distance, Position};
use rand::Rng;
use std::cmp::max;

fn smooth(start: f64, end: f64, j: f64) -> f64 {
    let t = (j - start) / (end - start) * std::f64::consts::PI;
    return t.sin().powi(2);
}

fn hypotenuse(a: Position, b: Position) -> f64 {
    return ((a.latitude - b.latitude).powi(2) + (a.longitude - b.longitude).powi(2)).sqrt();
}

pub fn random_route(loc_list: Vec<Position>, d: f64, parts: i32) -> Vec<Position> {
    const EPS: f64 = 1e-9;
    let mut result: Vec<Position> = loc_list.clone();
    let center = Position {
        latitude: loc_list.iter().map(|x| x.latitude).sum::<f64>() / loc_list.len() as f64,
        longitude: loc_list.iter().map(|x| x.longitude).sum::<f64>() / loc_list.len() as f64,
    };

    let mut rng = rand::rng();

    for i in 0..parts {
        let start = (i as f64 * loc_list.len() as f64 / parts as f64) as usize;
        let end = ((i + 1) as f64 * loc_list.len() as f64 / parts as f64) as usize;
        let offset = (2.0 * rng.random::<f64>() - 1.0) * d;
        for j in start..end {
            let dis = hypotenuse(result[j], center);
            // TODO: Check if dis == 0.0 is correct
            if dis.abs() < EPS {
                continue;
            }
            result[j].latitude += (result[j].latitude - center.latitude) as f64 / dis
                * offset
                * smooth(start as f64, end as f64, j as f64);
            result[j].longitude += (result[j].longitude - center.longitude) as f64 / dis
                * offset
                * smooth(start as f64, end as f64, j as f64);
        }
    }

    return result;
}

pub fn fixed_route(loc_list: Vec<Position>, v: f64, dt: f64) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::new();
    for i in 0..loc_list.len() {
        let start = loc_list[i];
        let end = loc_list[if i + 1 < loc_list.len() { i + 1 } else { 0 }];
        let delta_t = geo_distance(start, end) / v;
        let mut t = 0.0;
        let mut j = 0;
        while t < delta_t {
            let d_lat = (end.latitude - start.latitude) / max(1, (delta_t / dt) as i32) as f64;
            let d_lng = (end.longitude - start.longitude) / max(1, (delta_t / dt) as i32) as f64;
            let xa = start.latitude + d_lat * j as f64;
            let xb = start.longitude + d_lng * j as f64;
            result.push(Position {
                latitude: xa,
                longitude: xb,
            });
            j += 1;
            t += dt;
        }
    }
    return result;
}
