#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use core::{panic::PanicInfo, time::Duration};

use log::{debug, error};
use uefi::{CStr16, guid, prelude::*, println};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let mut name: [u16; _] = [0; 22];
    let fum_name = match CStr16::from_str_with_buf("FirmwareUpdateMode", &mut name) {
        Ok(cstr) => cstr,
        Err(e) => {
            return standard_error(&format!("Failed to convert FirmwareUpdateMode to CStr16: {}", e));
        },
    };
    let guid = uefi::runtime::VariableVendor(guid!("d15b327e-ff2d-4fc1-abf6-c12bd08c1359"));

    let mut fum_buf = [0; 256];
    debug!("Reading {}-{}", fum_name, guid.0);
    let (variable_buf, attr) = match uefi::runtime::get_variable(fum_name, &guid, &mut fum_buf) {
        Ok(v) => v,
        Err(e) => {
            match e.status() {
                Status::NOT_FOUND => {
                    println!("FirmwareUpdateMode variable not found");
                    return Status::SUCCESS
                },
                _ => {
                    return standard_error(&format!("Failed to read FirmwareUpdateMode variable: {}", e))
                }
            }
        },
    };
    debug!("Attributes: {:?}", attr);
    debug!("Deleting {}-{}", fum_name, guid.0);
    if let Err(e) = uefi::runtime::delete_variable(fum_name, &guid) {
        return standard_error(&format!("Failed to delete FirmwareUpdateMode variable: {}", e));
    }

    let fum_rt_name = match CStr16::from_str_with_buf("FirmwareUpdateModeRT", &mut name) {
        Ok(cstr) => cstr,
        Err(e) => {
            return standard_error(&format!("Failed to convert FirmwareUpdateModeRT to CStr16: {}", e));
        },
    };
    debug!("Setting new variable: {}-{}", fum_rt_name, guid.0);
    debug!("Variable attributes: {:?}", attr);
    if let Err(e) = uefi::runtime::set_variable(fum_rt_name, &guid, attr, &variable_buf) {
        return standard_error(&format!("Failed to create FirmwareUpdateModeRT: {}", e));
    }
    println!("Replaced FirmwareUpdateMode with FirmwareUpdateModeRT");
    Status::SUCCESS
}

fn standard_error(debug_err: &str) -> Status {
    debug!("{debug_err}");
    error!("Application failed to replace FirmwareUpdateMode EFI variable");
    error!("Exiting in 3 seconds.");
    boot::stall(Duration::from_secs(3));
    Status::UNSUPPORTED
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("Critical error!: {}", info.message());
    error!("Please reboot your system");
    loop {}
}
