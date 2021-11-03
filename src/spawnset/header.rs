use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;

pub struct Header {
    pub spawn_version: i32,
    pub world_version: i32,
    pub shrink_end: f32,
    pub shrink_start: f32,
    pub shrink_rate: f32,
    pub brightness: f32,
    pub game_mode: i32,
    _unknown_1: u32,
    _unknown_2: u32
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
            game_mode,
            _unknown_1: 0x00000033,
            _unknown_2: 0x00000001
        }
    }

    pub fn size() -> usize {
        36
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Result<Header, DDError> {
        let spawn_version = byte_reader.get_i32()?;
        let world_version = byte_reader.get_i32()?;
        let shrink_end = byte_reader.get_f32()?;
        let shrink_start = byte_reader.get_f32()?;
        let shrink_rate = byte_reader.get_f32()?;
        let brightness = byte_reader.get_f32()?;
        let game_mode = byte_reader.get_i32()?;

        byte_reader.skip_bytes(8);

        Ok(Self::new(spawn_version, world_version, shrink_end, shrink_start,
                  shrink_rate, brightness, game_mode))
    }

    pub fn to_byte_slice(&self, byte_slice: &mut [u8]) -> Result<(), DDError> {
        if byte_slice.len() < Self::size() {
            return Err(DDError::NotEnoughDataWrite)
        }

        byte_slice[0..4].copy_from_slice(&self.spawn_version.to_le_bytes());
        byte_slice[4..8].copy_from_slice(&self.world_version.to_le_bytes());
        byte_slice[8..12].copy_from_slice(&self.shrink_end.to_le_bytes());
        byte_slice[12..16].copy_from_slice(&self.shrink_start.to_le_bytes());
        byte_slice[16..20].copy_from_slice(&self.shrink_rate.to_le_bytes());
        byte_slice[20..24].copy_from_slice(&self.brightness.to_le_bytes());
        byte_slice[24..28].copy_from_slice(&self.game_mode.to_le_bytes());
        byte_slice[28..32].copy_from_slice(&self._unknown_1.to_le_bytes());
        byte_slice[32..36].copy_from_slice(&self._unknown_2.to_le_bytes());

        Ok(())
    }
}