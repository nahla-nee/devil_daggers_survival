use std::io::{Write, Read};
use super::utils::*;

#[cfg_attr(feature = "json_coding", derive(Serialize, Deserialize))]
pub struct Settings {
    pub initial_hand: i8,
    pub additional_gems: i32,
    pub time_start: f32
}

impl Settings {
    pub fn new(initial_hand: i8, additional_gems: i32, time_start: f32) -> Settings {
        Settings {
            initial_hand,
            additional_gems,
            time_start
        }
    }

    pub fn size(spawn_version: i32) -> usize {
        match spawn_version {
            4 => 0,
            5 => 5,
            6 => 9,
            _ => panic!("unexpected value passed to size_from_spawn_ver: {}", spawn_version)
        }
    }

    pub fn from_reader<R: Read>(spawn_version: i32, reader: &mut R) -> Settings {
        let mut settings= Settings::new(0, 0, 0f32);
        if spawn_version > 4 {
            settings.initial_hand = get_i8(reader);
            settings.additional_gems = get_i32(reader);

            if spawn_version > 5 {
                settings.time_start = get_f32(reader);
            }
        }

        settings
    }

    pub fn to_writer<W: Write>(&self, spawn_version: i32, writer: &mut W) {
        if spawn_version > 4 {
            let _ = writer.write(&self.initial_hand.to_le_bytes());
            let _ = writer.write(&self.additional_gems.to_le_bytes());
            if spawn_version > 5 {
                let _ = writer.write(&self.time_start.to_le_bytes());
            }
        }
    }
}