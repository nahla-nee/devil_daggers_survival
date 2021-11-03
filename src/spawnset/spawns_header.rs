use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;
use std::mem::size_of;

pub struct SpawnsHeader {
    _unknown_1: u32,
    _unknown_2: u32,
    _unknown_3: u32,
    _unknown_4: u32,
    pub devil_dagger_unlock_time: u32,
    pub golden_dagger_unlock_time: u32,
    pub silver_dagger_unlock_time: u32,
    pub bronze_dagger_unlock_time: u32,
    _unknown_5: u32,
    pub spawns_count: u32
}

impl SpawnsHeader {
    pub fn new(devil_dagger_unlock_time: u32, golden_dagger_unlock_time: u32,
               silver_dagger_unlock_time: u32, bronze_dagger_unlock_time: u32, spawns_count: u32)
        -> SpawnsHeader {
        SpawnsHeader {
            _unknown_1: 0,
            _unknown_2: 0,
            _unknown_3: 0,
            _unknown_4: 0x01000000,
            devil_dagger_unlock_time,
            golden_dagger_unlock_time,
            silver_dagger_unlock_time,
            bronze_dagger_unlock_time,
            _unknown_5: 0,
            spawns_count
        }
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Result<SpawnsHeader, DDError> {
        if byte_reader.bytes_left() < 40 {
            return Err(DDError::NotEnoughData)
        }

        //dump initial unknown values
        byte_reader.skip_bytes(size_of::<u32>()*4);

        let devil_dagger_unlock_time = byte_reader.get_u32().unwrap();
        let silver_dagger_unlock_time = byte_reader.get_u32().unwrap();
        let golden_dagger_unlock_time = byte_reader.get_u32().unwrap();
        let bronze_dagger_unlock_time = byte_reader.get_u32().unwrap();
        
        //dump one more unknown
        byte_reader.skip_bytes(size_of::<u32>());
        
        let spawns_count = byte_reader.get_u32().unwrap();

        Ok(Self::new(devil_dagger_unlock_time, golden_dagger_unlock_time,
            silver_dagger_unlock_time, bronze_dagger_unlock_time, spawns_count))
    }
}