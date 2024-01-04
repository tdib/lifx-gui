// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod device;
mod header;
mod packet;
mod payload;

use device::Device;
use packet::Packet;
use payload::Payload;
use std::net::{SocketAddr, UdpSocket};

use hex;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_power, set_colour])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn set_power(mac: &str, status: bool) {
    // println!("setting power");
    let mac_address_hex = u64::from_str_radix(mac, 16).unwrap();

    let payload = Some(Payload::set_power(status));

    // TODO: automatically extract protocol number
    let packet = Packet::new(Some(mac_address_hex), 21, payload);
    broadcast(packet)
}

#[tauri::command]
fn set_colour(mac: &str, hue: u16, saturation: u16, brightness: u16, kelvin: u16, duration: u32) {
    // println!("setting colour mac={mac} hue={hue} sat={saturation} bright={brightness} kelvin={kelvin} duration={duration}");
    let mac_address_hex = u64::from_str_radix(mac, 16).unwrap();

    let payload = Some(Payload::set_colour(
        hue, saturation, brightness, kelvin, duration,
    ));

    let packet = Packet::new(Some(mac_address_hex), 102, payload);
    broadcast(packet)
}

fn broadcast(packet: Packet) {
    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    // Enable broadcast option
    socket
        .set_broadcast(true)
        .expect("Failed to set broadcast option");

    // Destination address
    let broadcast_address = "192.168.1.255";
    let broadcast_port = 56700;
    let dest_addr: SocketAddr = format!("{}:{}", broadcast_address, broadcast_port)
        .parse()
        .expect("Failed to parse destination address");

    // Send the packet to the destination
    let packet = Vec::from(packet);
    socket
        .send_to(&packet, dest_addr)
        .expect("Failed to send packet");
}
