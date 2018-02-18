#![feature(proc_macro)]
extern crate boxcars;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use boxcars::{ParserBuilder, Replay};

#[wasm_bindgen]
#[derive(Default)]
pub struct WebReplay {
    header_size: i32,
    header_crc: i32,
    major_version: i32,
    minor_version: i32,
    game_type: String,
}

#[wasm_bindgen]
impl WebReplay {
	pub fn header_size(&self) -> i32 {
        self.header_size
    }	

	pub fn header_crc(&self) -> i32 {
        self.header_crc
    }

	pub fn major_version(&self) -> i32 {
        self.major_version
    }

	pub fn minor_version(&self) -> i32 {
        self.minor_version
    }

	pub fn game_type(&self) -> String {
        self.game_type.clone()
    }
}

impl<'a> From<Replay<'a>> for WebReplay {
    fn from(replay: Replay<'a>) -> Self {
        WebReplay {
            header_size: replay.header_size,
            header_crc: replay.header_crc,
            major_version: replay.major_version,
            minor_version: replay.minor_version,
            game_type: replay.game_type.to_owned().to_string(),
        }
    }
}

#[no_mangle]
#[wasm_bindgen]
pub extern fn parse_replay(data: &[u8]) -> WebReplay {
    let parsing = ParserBuilder::new(data)
        .always_check_crc()
        .must_parse_network_data()
        .parse();
    
    match parsing {
        Ok(replay) => WebReplay::from(replay),
        Err(ref e) => WebReplay::default(),
    }
}
