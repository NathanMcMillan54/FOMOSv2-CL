// fomos/src/cl/mod.rs
pub mod cl;
mod help;

extern "C" {
    fn cl_input();
}