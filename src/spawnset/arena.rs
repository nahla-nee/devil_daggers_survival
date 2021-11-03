use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;
use std::slice;

pub struct Arena {
    pub arena: [f32; 51*51]
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            arena: [0f32; 51*51]
        }
    }

    pub fn size() -> usize {
        51*51*4
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Result<Arena, DDError> {
        let mut arena = [0f32; 51*51];
        unsafe {
            byte_reader.get_chunk(arena.as_mut_ptr(), 51*51)?;
        }

        Ok(Arena {
            arena
        })
    }

    pub fn to_byte_slice(&self, byte_slice: &mut [u8]) -> Result<(), DDError> {
        if byte_slice.len() < Self::size() {
            return Err(DDError::NotEnoughDataWrite)
        }

        let arena = unsafe {
            let arena_ptr = self.arena.as_ptr() as *const u8;
            slice::from_raw_parts(arena_ptr, self.arena.len()*4)
        };
        byte_slice.copy_from_slice(arena);

        Ok(())
    }

    pub fn get_at(&self, x: usize, y: usize) -> f32 {
        self.arena[y*51+x]
    }

    pub fn set_at(&mut self, x: usize, y: usize, val: f32) {
        self.arena[y*51+x] = val;
    }
}
