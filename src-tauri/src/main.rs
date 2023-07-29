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
        .invoke_handler(tauri::generate_handler![set_power])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn set_power(status: bool) {
    // MAC address of target device
    let device_mac_hex = "d073d5xxxxxx";
    let device_mac_dec = u64::from_str_radix(device_mac_hex, 16).unwrap();

    // let payload = Some(Payload::set_colour(0x5555, 0xFFFF, 0xFFFF, 0xAC0D, 0x0));
    // let payload = Some(Payload::set_colour(0x0001, 0xFFFF, 0xFFFF, 0xAC0D, 0));
    let payload = Some(Payload::set_power(status));
    dbg!(&payload);

    // TODO: automatically extract protocol number
    let packet = Packet::new(Some(device_mac_dec), 21, payload);
    // let packet = Packet::new(Some(device_mac_dec), 102, payload);
    // dbg!(&packet);

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

    let packet = Vec::from(packet);
    // Send the packet to the destination
    socket
        .send_to(&packet, dest_addr)
        .expect("Failed to send packet");
}
