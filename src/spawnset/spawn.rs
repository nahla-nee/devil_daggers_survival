use std::io::{Write, Read, Seek, SeekFrom};
use super::enemy_type::EnemyType;
use super::utils::*;

#[cfg(feature = "json_coding")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "json_coding", derive(Serialize, Deserialize))]
pub struct Spawn {
    pub enemy_type: EnemyType,
    pub spawn_delay: f32,
    _unknown_1: i32,
    _unknown_2: i32,
    _unknown_3: i32,
    _unknown_4: i32,
    _unknown_5: i32
}

impl Spawn {
    pub fn new(enemy_type: EnemyType, spawn_delay: f32) -> Spawn {
        Spawn {
            enemy_type,
            spawn_delay,
            _unknown_1: 0,
            _unknown_2: 0x00000003,
            _unknown_3: 0,
            _unknown_4: 0x41F00000,
            _unknown_5: 0x0000000A
        }
    }

    pub fn size() -> usize {
        28
    }

    pub fn from_reader<R: Read + Seek>(reader: &mut R) -> Spawn {
        let enemy_type = EnemyType::from_i32(get_i32(reader));
        let spawn_delay = get_f32(reader);

        let _ = reader.seek(SeekFrom::Current(20));

        Spawn::new(enemy_type, spawn_delay)
    }

    pub fn to_writer<W: Write>(&self, writer: &mut W) {
        let _ = writer.write(&self.enemy_type.to_i32().to_le_bytes());
        let _ = writer.write(&self.spawn_delay.to_le_bytes());

        let _ = writer.write(&0u32.to_le_bytes());
        let _ = writer.write(&0x00000003u32.to_le_bytes());
        let _ = writer.write(&0u32.to_le_bytes());
        let _ = writer.write(&0x41F00000u32.to_le_bytes());
        let _ = writer.write(&0x0000000Au32.to_le_bytes());
    }
}
