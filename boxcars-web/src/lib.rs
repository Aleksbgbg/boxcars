#![feature(proc_macro)]
extern crate boxcars;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use boxcars::{ParserBuilder, Replay};

#[wasm_bindgen]
pub struct KeyFrame {
    time: f32,
    frame: i32,
    position: i32,
}

#[wasm_bindgen]
impl KeyFrame {
    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn frame(&self) -> i32 {
        self.frame
    }

    pub fn position(&self) -> i32 {
        self.position
    }
}

#[wasm_bindgen]
pub struct Goal {
    frame: i32,
    team: String,
    player: String,
}

#[wasm_bindgen]
impl Goal {
    pub fn frame(&self) -> i32 {
        self.frame
    }

    pub fn team(&self) -> String {
        self.team.clone()
    }

    pub fn player(&self) -> String {
        self.player.clone()
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct WebReplay {
    header_size: i32,
    header_crc: i32,
    major_version: i32,
    minor_version: i32,
    game_type: String,
    key_frames: Vec<KeyFrame>,
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

    pub fn goals(&self) -> Vec<Goal> {
        vec![]
    }

    pub fn key_frames(&self) -> Vec<KeyFrame> {
        self.key_frames.clone()
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
            key_frames: replay.keyframes.into_iter().map(|x| KeyFrame {
                time: x.time,
                frame: x.frame,
                position: x.position,
            }).collect(),
        }
    }
}

#[no_mangle]
#[wasm_bindgen]
pub extern "C" fn parse_replay(data: &[u8]) -> WebReplay {
    let parsing = ParserBuilder::new(data)
        .always_check_crc()
        .must_parse_network_data()
        .parse();

    match parsing {
        Ok(replay) => WebReplay::from(replay),
        Err(ref e) => WebReplay::default(),
    }
}
