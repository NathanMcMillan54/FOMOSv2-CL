use crate::fomos_err::setup_err;
use serde::{Serialize, Deserialize};
extern crate serde_json;

use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct SetupFile {
    first_time_setup: bool,
    setup: bool
}

pub(crate) fn setup_fomos() {
    let mut setupFile = File::open("configurations/FOMOSv2_setup.json").unwrap();
    let mut contents = String::new();
    setupFile.read_to_string(&mut contents).unwrap();

    let setup: SetupFile = serde_json::from_str(&contents).unwrap();
    if setup.first_time_setup == true {
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

