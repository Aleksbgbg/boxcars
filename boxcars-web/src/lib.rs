#![feature(proc_macro)]
extern crate boxcars;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use boxcars::{ParserBuilder};
use std::slice;

/*
#[wasm_bindgen]
#[derive(Default)]
pub struct WebReplay {
    pub header_size: i32,
    pub header_crc: i32,
    pub major_version: i32,
    pub minor_version: i32,
}

impl WebReplay {
    pub fn new(int: i32) -> WebReplay {
        WebReplay {
            header_size: int,
        }
    }
}

impl<'a> From<Replay<'a>> for WebReplay {
    fn from(replay: Replay<'a>) -> Self {
        WebReplay {
            header_size: replay.header_size,
            header_crc: replay.header_crc,
            major_version: replay.major_version,
            minor_version: replay.minor_version,
        }
    }
}*/

#[no_mangle]
#[wasm_bindgen]
pub extern fn parse_replay(ptr: *mut u8, len: usize) -> String {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    let parsing = ParserBuilder::new(&slice[..])
        .always_check_crc()
        .must_parse_network_data()
        .parse();
    
    match parsing {
        Ok(replay) => format!("We did it: {}", replay.major_version),
        Err(ref e) => format!("error: {}", e),
    }
}
