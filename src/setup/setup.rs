#![no_std]
use serde_json::de::from_reader;

pub(crate) fn setup_fomos() {
    let setup_file = fs::File::open("configurations/FOMOSv2_setup.json").expect(fomos_err::setup_err::unreadable_111);

    let read_setup_file = serde_json::Value = serde_json::from_reader(setup_file);

    let first_time_setup = read_setup_file("first-time_setup");

    if first_time_setup = true {
        first-time_fomos_setup()
    } else {
        regular_setup();
    }
}

fn first_time_fomos_setup() {
    // Setup FOMOSV2-CL v2.3.5 for the first time
    // Restart device then run regular_setup()
    regular_setup();
}

fn regular_setup() {
    // Setup FOMOSV2-CL v2.3.5
}

