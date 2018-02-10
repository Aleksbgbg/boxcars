#![feature(proc_macro)]
extern crate boxcars;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use boxcars::ParserBuilder;
use std::slice;

#[no_mangle]
#[wasm_bindgen]
pub extern fn parse_replay(ptr: *mut u8, len: usize) -> String {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    let parsing = ParserBuilder::new(&slice[..])
        .always_check_crc()
        .must_parse_network_data()
        .parse();
    
    match parsing {
        Ok(_) => String::from("We did it!"),
        Err(ref e) => format!("error: {}", e),
    }
}
