use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;

pub struct Settings {
    pub initial_hand: i8,
    pub additional_gems: i32,
    pub time_start: Option<f32>
}

impl Settings {
    pub fn new(initial_hand: i8, additional_gems: i32, time_start: Option<f32>) -> Settings {
        Settings {
            initial_hand,
            additional_gems,
            time_start
        }
    }

    pub fn size_from_spawn_ver(spawn_ver: i32) -> usize {
        match spawn_ver {
            4 => 0,
            5 => 5,
            6 => 9,
            _ => panic!("unexpected value passed to size_from_spawn_ver: {}", spawn_ver)
        }
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader, spawn_ver: i32) -> Result<Option<Settings>, DDError> {
        let extended_ver = match spawn_ver {
            4 => return Ok(None),
            5 => false,
            6 => true,
            _ => panic!("unexpected value passed to from_byte_reader: {}", spawn_ver)
        };

        let initial_hand = byte_reader.get_i8()?;
        let additional_gems = byte_reader.get_i32()?;
        let time_start = if extended_ver {
            Some(byte_reader.get_f32()?)
        }
        else {
            None
        };

        Ok(Some(Self::new(initial_hand, additional_gems, time_start)))
    }

    pub fn to_byte_slice(&self, byte_slice: &mut [u8], spawn_ver: i32) -> Result<(), DDError> {
        let extended_ver = match spawn_ver {
            4 => return Ok(()),
            5 => false,
            6 => true,
            _ => panic!("unexpected value passed to to_byte_slice: {}", spawn_ver)
        };

        if byte_slice.len() < Self::size_from_spawn_ver(spawn_ver) {
            return Err(DDError::NotEnoughDataWrite)
        }

        byte_slice[0] = self.initial_hand as u8;
        byte_slice[1..5].copy_from_slice(&self.additional_gems.to_le_bytes());
        if extended_ver {
            byte_slice[5..9].copy_from_slice(&self.time_start.unwrap().to_le_bytes());
        }

        Ok(())
    }
}