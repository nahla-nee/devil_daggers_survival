use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;
use super::enemy_type::EnemyType;
use std::mem::size_of;

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

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Result<Spawn, DDError> {
        let enemy_type = EnemyType::from_i32(byte_reader.get_i32()?);
        let spawn_delay = byte_reader.get_f32()?;

        //dump unknowns
        byte_reader.skip_bytes(size_of::<i32>()*5);

        Ok(Self::new(enemy_type, spawn_delay))
    }

    pub fn to_byte_slice(&self, byte_slice: &mut [u8]) -> Result<(), DDError> {
        if byte_slice.len() < Self::size() {
            return Err(DDError::NotEnoughDataWrite)
        }

        byte_slice[0..4].copy_from_slice(&self.enemy_type.to_i32().to_le_bytes());
        byte_slice[4..8].copy_from_slice(&self.spawn_delay.to_le_bytes());
        byte_slice[8..12].copy_from_slice(&self._unknown_1.to_le_bytes());
        byte_slice[12..16].copy_from_slice(&self._unknown_2.to_le_bytes());
        byte_slice[16..20].copy_from_slice(&self._unknown_3.to_le_bytes());
        byte_slice[20..24].copy_from_slice(&self._unknown_4.to_le_bytes());
        byte_slice[24..28].copy_from_slice(&self._unknown_5.to_le_bytes());

        Ok(())
    }
}
