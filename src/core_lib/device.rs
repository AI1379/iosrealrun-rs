//
// Created by Renatus Madrigal on 02/03/2025
//

use rusty_libimobiledevice::{
    idevice::{self, Device},
    service::ServiceClient,
};

use super::coordinate::Position;

pub fn get_devices() -> Vec<Device> {
    match idevice::get_devices() {
        Ok(devices) => {
            return devices;
        }
        Err(e) => {
            panic!("Find devices error: {}!", e);
        }
    }
}

pub fn start_location_simulation(device: &Device) -> ServiceClient {
    let mut lockdown_client = match device.new_lockdownd_client("idevicelocation") {
        Ok(client) => client,
        Err(e) => {
            panic!("Lockdownd client error: {}!", e);
        }
    };
    let service = match lockdown_client.start_service("com.apple.dt.simulatelocation", false) {
        Ok(service) => service,
        Err(e) => {
            panic!("Start service error: {}!", e);
        }
    };
    return match ServiceClient::new(&device, service) {
        Ok(client) => client,
        Err(e) => {
            panic!("Service client error: {}!", e);
        }
    };
}

pub fn set_location(client: &ServiceClient, pos: Position) {
    let head = [0, 0, 0, 0].to_vec();
    let lat_len = (pos.lat.to_string().len() as u32)
        .to_be_bytes()
        .to_vec();
    let lat_str = pos.lat.to_string().as_bytes().to_vec();
    let lon_len = (pos.lng.to_string().len() as u32)
        .to_be_bytes()
        .to_vec();
    let lon_str = pos.lng.to_string().as_bytes().to_vec();
    let msg = [head, lat_len, lat_str, lon_len, lon_str].concat();
    match client.send(msg) {
        Ok(_) => {}
        Err(e) => {
            panic!("Set location error: {}!", e);
        }
    }
}

pub fn stop_location_simulation(client: &ServiceClient) {
    match client.send([0, 0, 0, 1].to_vec()) {
        Ok(_) => {}
        Err(e) => {
            panic!("Stop location simulation error: {}!", e);
        }
    }
}
