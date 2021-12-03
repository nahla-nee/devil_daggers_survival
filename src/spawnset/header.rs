use std::io::{Write, Read, Seek, SeekFrom};
use super::utils::*;

#[cfg(feature = "json_coding")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "json_coding", derive(Serialize, Deserialize))]
pub struct Header {
    pub spawn_version: i32,
    pub world_version: i32,
    pub shrink_end: f32,
    pub shrink_start: f32,
    pub shrink_rate: f32,
    pub brightness: f32,
    pub game_mode: i32
}

impl Header {
    pub fn new(spawn_version: i32, world_version: i32, shrink_end: f32, shrink_start: f32,
               shrink_rate: f32, brightness: f32, game_mode: i32) -> Header {
        Header {
            spawn_version,
            world_version,
            shrink_end,
            shrink_start,
            shrink_rate,
            brightness,
            game_mode
        }
    }

    pub fn size() -> usize {
        36
    }

    pub fn from_reader<R: Read + Seek>(reader: &mut R) -> Header {
        let spawn_version = get_i32(reader);
        let world_version = get_i32(reader);
        let shrink_end = get_f32(reader);
        let shrink_start = get_f32(reader);
        let shrink_rate = get_f32(reader);
        let brightness = get_f32(reader);
        let game_mode = get_i32(reader);

        let _ = reader.seek(SeekFrom::Current(8));

        Self::new(spawn_version, world_version, shrink_end, shrink_start,
                  shrink_rate, brightness, game_mode)
    }

    pub fn to_writer<W: Write>(&self, writer: &mut W) {
        let _ = writer.write(&self.spawn_version.to_le_bytes());
        let _ = writer.write(&self.world_version.to_le_bytes());
        let _ = writer.write(&self.shrink_end.to_le_bytes());
        let _ = writer.write(&self.shrink_start.to_le_bytes());
        let _ = writer.write(&self.shrink_rate.to_le_bytes());
        let _ = writer.write(&self.brightness.to_le_bytes());
        let _ = writer.write(&self.game_mode.to_le_bytes());
        let _ = writer.write(&0x00000033u32.to_le_bytes());
        let _ = writer.write(&0x00000001u32.to_le_bytes());
    }
}