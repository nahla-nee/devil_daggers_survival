use std::mem::size_of;
use crate::byte_reader::ByteReader;
use super::enemy_type::EnemyType;

pub struct Spawn {
    pub enemy_type: EnemyType,
    pub spawn_delay: f32,
    _unknown_1: u32,
    _unknown_2: u32,
    _unknown_3: u32,
    _unknown_4: u32,
    _unknown_5: u32
}

impl Spawn {
    pub fn new(enemy_type: EnemyType, spawn_delay: f32) -> Spawn {
        Spawn {
            enemy_type,
            spawn_delay,
            _unknown_1: 0x0,
            _unknown_2: 0x03000000,
            _unknown_3: 0x00000000,
            _unknown_4: 0x0000F041,
            _unknown_5: 0x0A000000
        }
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Spawn {
        if byte_reader.bytes_left() < 28 {
        }

        let enemy_type = EnemyType::from_i32(byte_reader.get_i32().unwrap());
        let spawn_delay = byte_reader.get_f32().unwrap();

        //dump unknowns
        byte_reader.skip_bytes(size_of::<u32>()*5);

        Spawn::new(enemy_type, spawn_delay)
    }
}
