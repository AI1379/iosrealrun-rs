//
// Created by Renatus Madrigal on 02/03/2025
//

use core_lib::coordinate::Position;
use rusty_libimobiledevice::idevice;
use serde::{Deserialize, Serialize};
use std::{cmp::min, path, println};

mod core_lib;

#[derive(Serialize, Deserialize)]
struct Config {
    speed: f64,
    interval: f64,
    route: Vec<Position>,
}

#[derive(PartialEq, Eq)]
enum Operation {
    ListDevices,
    Run,
    Help,
}

fn list_devices(_: &[String]) {
    let devices = core_lib::device::get_devices();
    for device in devices {
        println!("Device: {}", device.get_udid());
    }
}

fn run(args: &[String]) {
    let mut idx = 0;
    let mut udid = idevice::get_first_device().unwrap().get_udid();
    let mut config_path = "./config.json";
    let mut loop_count = 5;
    while idx < args.len() {
        match args[idx].as_str() {
            "-u" | "--udid" => {
                idx += 1;
                udid = args[idx].to_string();
            }
            "-c" | "--config" => {
                idx += 1;
                config_path = &args[idx];
            }
            "-n" | "--count" => {
                idx += 1;
                loop_count = args[idx].parse().unwrap();
            }
            _ => {
                println!("Invalid option: {}", args[idx]);
                return;
            }
        }
        idx += 1;
    }
    println!("Device UDID: {}", udid);
    println!("Config file path: {}", config_path);
    println!("Loop count: {}", loop_count);
    let config_data = std::fs::read_to_string(config_path)
        .expect(&format!("Failed to read config file at {}", config_path));
    let config: Config = serde_json::from_str(config_data.as_str())
        .expect(&format!("Failed to parse config data: {}", config_data));
    println!("Speed: {}m/s", config.speed);
    println!("Interval: {}s", config.interval);
    let device = match idevice::get_device(udid.as_str()) {
        Ok(device) => device,
        Err(e) => {
            panic!("Find device error: {}!", e);
        }
    };
    let client = core_lib::device::start_location_simulation(&device);
    core_lib::run::run(
        &client,
        config.route,
        config.speed,
        config.interval,
        loop_count,
    );
    core_lib::device::stop_location_simulation(&client);
}

fn help(cmd: &String) {
    let runnable = cmd.split(path::MAIN_SEPARATOR).last().unwrap();
    println!("Usage: {} [mode] [options]", runnable);
    println!();
    println!("Modes:");
    println!("  list\t\tList all connected devices");
    println!("  run\t\tStart location simulation");
    println!("  help\t\tPrint this help message");
    println!();
    println!("Options of run:");
    println!("  -u, --udid\tDevice udid");
    println!("  -c, --config\tConfig file path");
    println!("  -n, --count\tLoop count");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let operation = if args.len() > 1 {
        match args[1].as_str() {
            "list" => Operation::ListDevices,
            "run" => Operation::Run,
            "help" => Operation::Help,
            _ => {
                println!("Invalid mode: {}", args[1]);
                return;
            }
        }
    } else {
        Operation::Run
    };
    match operation {
        Operation::Help => help(&args[0]),
        Operation::ListDevices => list_devices(&args[2..]),
        Operation::Run => run(&args[min(args.len(), 2)..]),
    }
}
