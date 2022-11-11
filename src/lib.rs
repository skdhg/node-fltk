use fltk::{app as fltkapp};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn version() -> String {
    fltkapp::version_str()
}

mod app;