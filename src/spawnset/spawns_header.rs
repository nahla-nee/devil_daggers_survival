use crate::byte_reader::ByteReader;
use std::mem::size_of;

pub struct SpawnsHeader {
    _unknown_1: u32,
    _unknown_2: u32,
    _unknown_3: u32,
    _unknown_4: u32,
    devil_dagger_unlock_time: u32,
    golden_dagger_unlock_time: u32,
    silver_dagger_unlock_time: u32,
    bronze_dagger_unlock_time: u32,
    _unknown_5: u32,
    spawns_count: u32
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

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> SpawnsHeader {
        if byte_reader.bytes_left() < 40 {
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

        Self::new(devil_dagger_unlock_time, golden_dagger_unlock_time,
            silver_dagger_unlock_time, bronze_dagger_unlock_time, spawns_count)
    }

    pub fn get_devil_dagger_unlock_time(&self) -> u32 {
        self.devil_dagger_unlock_time
    }

    pub fn get_golden_dagger_unlock_time(&self) -> u32 {
        self.golden_dagger_unlock_time
    }

    pub fn get_silver_dagger_unlock_time(&self) -> u32 {
        self.silver_dagger_unlock_time
    }

    pub fn get_bronze_dagger_unlock_time(&self) -> u32 {
        self.bronze_dagger_unlock_time
    }

    pub fn get_spawns_count(&self) -> u32 {
        self.spawns_count
    }

    pub fn set_devil_dagger_unlock_time(&mut self, val: u32) {
        self.devil_dagger_unlock_time = val
    }

    pub fn set_golden_dagger_unlock_time(&mut self, val: u32) {
        self.golden_dagger_unlock_time = val
    }

    pub fn set_silver_dagger_unlock_time(&mut self, val: u32) {
        self.silver_dagger_unlock_time = val
    }

    pub fn set_bronze_dagger_unlock_time(&mut self, val: u32) {
        self.bronze_dagger_unlock_time = val
    }

    pub fn set_spawns_count(&mut self, val: u32) {
        self.spawns_count = val
    }
}