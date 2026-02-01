// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod usb_device;
mod audio_monitor;

use usb_device::EvoDevice;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    device: Mutex<Option<EvoDevice>>,
}

#[tauri::command]
fn set_gain(state: State<AppState>, channel: u8, gain: u8) -> Result<(), String> {
    let guard = state.device.lock().unwrap();
    if let Some(dev) = guard.as_ref() {
        dev.set_input_gain(channel, gain)
    } else {
        Err("Device not initialized".to_string())
    }
}

#[tauri::command]
fn toggle_phantom(state: State<AppState>, channel: u8, enabled: bool) -> Result<(), String> {
    let guard = state.device.lock().unwrap();
    if let Some(dev) = guard.as_ref() {
        dev.set_phantom_power(channel, enabled)
    } else {
        Err("Device not initialized".to_string())
    }
}

#[tauri::command]
fn set_master_volume(state: State<AppState>, volume: u8) -> Result<(), String> {
    let guard = state.device.lock().unwrap();
    if let Some(dev) = guard.as_ref() {
        dev.set_output_volume(volume)
    } else {
        Err("Device not initialized".to_string())
    }
}

#[tauri::command]
fn set_mixer_node(state: State<AppState>, node_index: u8, volume_db: f32) -> Result<(), String> {
    let guard = state.device.lock().unwrap();
    if let Some(dev) = guard.as_ref() {
        dev.set_mixer_node(node_index, volume_db)
    } else {
        Err("Device not initialized".to_string())
    }
}

fn main() {
    let device = EvoDevice::open().ok();
    
    tauri::Builder::default()
        .manage(AppState {
            device: Mutex::new(device),
        })
        .invoke_handler(tauri::generate_handler![
            set_gain,
            toggle_phantom,
            set_master_volume,
            set_mixer_node
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
