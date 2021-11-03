use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;
use std::mem::size_of;

pub struct SpawnsHeader {
    _unknown_1: i32,
    _unknown_2: i32,
    _unknown_3: i32,
    _unknown_4: i32,
    pub devil_dagger_unlock_time: i32,
    pub golden_dagger_unlock_time: i32,
    pub silver_dagger_unlock_time: i32,
    pub bronze_dagger_unlock_time: i32,
    _unknown_5: i32,
    pub spawns_count: i32
}

impl SpawnsHeader {
    pub fn new(devil_dagger_unlock_time: i32, golden_dagger_unlock_time: i32,
               silver_dagger_unlock_time: i32, bronze_dagger_unlock_time: i32, spawns_count: i32)
        -> SpawnsHeader {
        SpawnsHeader {
            _unknown_1: 0,
            _unknown_2: 0,
            _unknown_3: 0,
            _unknown_4: 0x00000001,
            devil_dagger_unlock_time,
            golden_dagger_unlock_time,
            silver_dagger_unlock_time,
            bronze_dagger_unlock_time,
            _unknown_5: 0,
            spawns_count
        }
    }

    pub fn size() -> usize {
        40
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Result<SpawnsHeader, DDError> {
        //dump initial unknown values
        byte_reader.skip_bytes(size_of::<i32>()*4);

        let devil_dagger_unlock_time = byte_reader.get_i32()?;
        let golden_dagger_unlock_time = byte_reader.get_i32()?;
        let silver_dagger_unlock_time = byte_reader.get_i32()?;
        let bronze_dagger_unlock_time = byte_reader.get_i32()?;
        
        //dump one more unknown
        byte_reader.skip_bytes(size_of::<i32>());
        
        let spawns_count = byte_reader.get_i32()?;

        Ok(Self::new(devil_dagger_unlock_time, golden_dagger_unlock_time,
            silver_dagger_unlock_time, bronze_dagger_unlock_time, spawns_count))
    }

    pub fn to_byte_slice(&self, byte_slice: &mut [u8]) -> Result<(), DDError> {
        if byte_slice.len() < Self::size() {
            return Err(DDError::NotEnoughDataWrite)
        }

        byte_slice[0..4].copy_from_slice(&self._unknown_1.to_le_bytes());
        byte_slice[4..8].copy_from_slice(&self._unknown_2.to_le_bytes());
        byte_slice[8..12].copy_from_slice(&self._unknown_3.to_le_bytes());
        byte_slice[12..16].copy_from_slice(&self._unknown_4.to_le_bytes());
        byte_slice[16..20].copy_from_slice(&self.devil_dagger_unlock_time.to_le_bytes());
        byte_slice[20..24].copy_from_slice(&self.golden_dagger_unlock_time.to_le_bytes());
        byte_slice[24..28].copy_from_slice(&self.silver_dagger_unlock_time.to_le_bytes());
        byte_slice[28..32].copy_from_slice(&self.bronze_dagger_unlock_time.to_le_bytes());
        byte_slice[32..36].copy_from_slice(&self._unknown_5.to_le_bytes());
        byte_slice[36..40].copy_from_slice(&self.spawns_count.to_le_bytes());

        Ok(())
    }
}