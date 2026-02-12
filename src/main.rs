#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

use core::{panic::PanicInfo, time::Duration};

use log::{debug, error};
use uefi::{CStr16, guid, prelude::*, println};

const FUM_NAME: &str = "FirmwareUpdateMode";
const FUM_RT_NAME: &str = "FirmwareUpdateModeRT";
/// https://github.com/Dasharo/edk2/blob/edbff52d39d1420a22cc4df8b56d8e78dd43fce4/DasharoModulePkg/Library/DasharoVariablesLib/DasharoVariablesLib.c#L573-L579
const DASHARO_GUID: uefi::Guid = guid!("d15b327e-ff2d-4fc1-abf6-c12bd08c1359");
const DASHARO_VENDOR: uefi::runtime::VariableVendor = uefi::runtime::VariableVendor(DASHARO_GUID);

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let mut name: [u16; _] = [0; 22];
    let cstr_fum_name = match CStr16::from_str_with_buf(FUM_NAME, &mut name) {
        Ok(cstr) => cstr,
        Err(e) => {
            return standard_error(&format!("Failed to convert {FUM_NAME} to CStr16: {e}"));
        }
    };

    let mut fum_buf = [0; 256];
    debug!("Reading {FUM_NAME}-{DASHARO_GUID}");
    let (variable_buf, attr) =
        match uefi::runtime::get_variable(cstr_fum_name, &DASHARO_VENDOR, &mut fum_buf) {
            Ok(v) => v,
            Err(e) => match e.status() {
                Status::NOT_FOUND => {
                    println!("{FUM_NAME} variable not found");
                    return Status::SUCCESS;
                }
                _ => return standard_error(&format!("Failed to read {FUM_NAME} variable: {e}")),
            },
        };
    debug!("Attributes: {:?}", attr);
    debug!("Deleting {FUM_NAME}-{DASHARO_GUID}");
    if let Err(e) = uefi::runtime::delete_variable(cstr_fum_name, &DASHARO_VENDOR) {
        return standard_error(&format!("Failed to delete {FUM_NAME} variable: {e}"));
    }

    let cstr_fum_rt_name = match CStr16::from_str_with_buf(FUM_RT_NAME, &mut name) {
        Ok(cstr) => cstr,
        Err(e) => {
            return standard_error(&format!("Failed to convert {FUM_RT_NAME} to CStr16: {e}"));
        }
    };
    debug!("Setting new variable: {FUM_RT_NAME}-{DASHARO_GUID}");
    debug!("Variable attributes: {:?}", attr);
    if let Err(e) =
        uefi::runtime::set_variable(cstr_fum_rt_name, &DASHARO_VENDOR, attr, &variable_buf)
    {
        return standard_error(&format!("Failed to create {FUM_RT_NAME}: {e}"));
    }
    println!("Replaced {FUM_NAME} with {FUM_RT_NAME}");
    Status::SUCCESS
}

fn standard_error(debug_err: &str) -> Status {
    debug!("{debug_err}");
    error!("Application failed to replace {FUM_NAME} EFI variable");
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
