use crate::fomos_err::setup_err;
use serde_json::*;
use std::fs;
use std::fs::File;
use std::io::Read;

pub(crate) fn setup_fomos() {
    let mut setup_file = File::open("configurations/FOMOSv2_setup.json").unwrap();
    let mut setup_file_data = String::new();
    setup_file.read_to_string(&mut setup_file_data).unwrap();
    let setup_file_json = serde_json::from_str(&setup_file_data).unwrap();

    if setup_file_json.find_path(&["first-time_setup"]).unwrap() == true {
        first_time_fomos_setup();
    } else {
        regular_setup();
    }
}

pub(crate) fn finish_setup_linux() {
    // Finish setting up the kernel
}

fn first_time_fomos_setup() {
    // Setup FOMOSV2-CL v2.3.5 for the first time
    // Restart device then run regular_setup()
    regular_setup();
}

fn regular_setup() {
    // Setup FOMOSV2-CL v2.3.5
}

