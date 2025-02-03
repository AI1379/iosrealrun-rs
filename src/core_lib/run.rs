//
// Created by Renatus Madrigal on 02/03/2025
//

use rand::{
    distr::{uniform::UniformFloat, Distribution, Uniform},
    Rng,
};
use rusty_libimobiledevice::service::ServiceClient;

use super::{
    coordinate::Position,
    route::{fixed_route, random_route},
};

const PART_NUM_LIST: [i32; 5] = [5, 6, 7, 8, 9];

fn run_loop(client: &ServiceClient, loc_list: Vec<Position>, v: f64, dt: f64) {
    let fixed_loc = fixed_route(loc_list, v, dt);
    let parts = PART_NUM_LIST[(rand::random::<u32>() % (PART_NUM_LIST.len() as u32)) as usize];
    let route = random_route(fixed_loc, 0.000025, parts);
    let mut clock = std::time::SystemTime::now();
    for pos in route {
        let now = std::time::SystemTime::now();
        let elapsed = now.duration_since(clock).unwrap();
        if elapsed.as_secs_f64() < dt {
            std::thread::sleep(std::time::Duration::from_secs_f64(
                dt - elapsed.as_secs_f64(),
            ));
        }
        clock = std::time::SystemTime::now();
        super::device::set_location(client, pos);
    }
}

const DEFAULT_DT: f64 = 0.2;

pub fn run(client: &ServiceClient, loc_list: Vec<Position>, v: f64, dt: f64, loop_count: i32) {
    if dt == 0.0 {
        run(client, loc_list, v, DEFAULT_DT, loop_count);
        return;
    }
    let dist = Uniform::<f64>::new(0.9, 1.1).unwrap();
    let mut rng = rand::rng();
    for _ in 0..loop_count {
        let rand_v = v * dist.sample(&mut rng);
        run_loop(client, loc_list.clone(), rand_v, dt);
    }
}
