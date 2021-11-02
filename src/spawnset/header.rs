use crate::byte_reader::ByteReader;
use std::mem::size_of;

pub struct Header {
    spawn_version: u32,
    world_version: u32,
    shrink_end: f32,
    shrink_start: f32,
    shrink_rate: f32,
    brightness: f32,
    game_mode: u32,
    _unknown_1: u32,
    _unknown_2: u32
}

impl Header {
    pub fn new(spawn_version: u32, shrink_end: f32, shrink_start: f32,
               shrink_rate: f32, brightness: f32, game_mode: u32) -> Header {
        Header {
            spawn_version,
            world_version: 9,
            shrink_end,
            shrink_start,
            shrink_rate,
            brightness,
            game_mode,
            _unknown_1: 0x33000000,
            _unknown_2: 0x01000000
        }
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Header {
        if byte_reader.bytes_left() < 36 {
        }

        let spawn_version = byte_reader.get_u32().unwrap();
        let _world_version = byte_reader.get_u32().unwrap();
        let shrink_end = byte_reader.get_f32().unwrap();
        let shrink_start = byte_reader.get_f32().unwrap();
        let shrink_rate = byte_reader.get_f32().unwrap();
        let brightness = byte_reader.get_f32().unwrap();
        let game_mode = byte_reader.get_u32().unwrap();

        byte_reader.skip_bytes(size_of::<u32>()*2);

        Self::new(spawn_version, shrink_end, shrink_start,
                  shrink_rate, brightness, game_mode)
    }

    pub fn get_spawn_version(&self) -> u32 {
        self.spawn_version
    }

    pub fn get_world_version(&self) -> u32 {
        self.world_version
    }

    pub fn get_shrink_end(&self) -> f32 {
        self.shrink_end
    }

    pub fn get_shrink_start(&self) -> f32 {
        self.shrink_start
    }

    pub fn get_shrink_rate(&self) -> f32 {
        self.shrink_rate
    }

    pub fn get_brightness(&self) -> f32 {
        self.brightness
    }

    pub fn get_game_mode(&self) -> u32 {
        self.game_mode
    }

    pub fn set_spawn_version(&mut self, val: u32) {
        self.spawn_version = val
    }

    pub fn set_shrink_end(&mut self, val: f32) {
        self.shrink_end = val
    }

    pub fn set_shrink_start(&mut self, val: f32) {
        self.shrink_start = val
    }

    pub fn set_shrink_rate(&mut self, val: f32) {
        self.shrink_rate = val
    }

    pub fn set_brightness(&mut self, val: f32) {
        self.brightness = val
    }
}