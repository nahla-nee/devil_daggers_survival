use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;
use std::mem::size_of;

pub struct Header {
    pub spawn_version: u32,
    pub world_version: u32,
    pub shrink_end: f32,
    pub shrink_start: f32,
    pub shrink_rate: f32,
    pub brightness: f32,
    pub game_mode: u32,
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

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Result<Header, DDError> {
        if byte_reader.bytes_left() < 36 {
            return Err(DDError::NotEnoughData)
        }

        let spawn_version = byte_reader.get_u32().unwrap();
        let _world_version = byte_reader.get_u32().unwrap();
        let shrink_end = byte_reader.get_f32().unwrap();
        let shrink_start = byte_reader.get_f32().unwrap();
        let shrink_rate = byte_reader.get_f32().unwrap();
        let brightness = byte_reader.get_f32().unwrap();
        let game_mode = byte_reader.get_u32().unwrap();

        byte_reader.skip_bytes(size_of::<u32>()*2);

        Ok(Self::new(spawn_version, shrink_end, shrink_start,
                  shrink_rate, brightness, game_mode))
    }
}