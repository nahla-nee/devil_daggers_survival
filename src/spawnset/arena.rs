use crate::byte_reader::ByteReader;
use crate::dd_error::DDError;

pub struct Arena {
    pub arena: [f32; 2601]
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            arena: [0f32; 2601]
        }
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Result<Arena, DDError> {
        let mut arena = [0f32; 2601];
        unsafe {
            byte_reader.get_chunk(arena.as_mut_ptr(), 2601)?
        }

        Ok(Arena {
            arena
        })
    }

    pub fn get_at(&self, x: usize, y: usize) -> f32 {
        self.arena[y*51+x]
    }

    pub fn set_at(&mut self, x: usize, y: usize, val: f32) {
        self.arena[y*51+x] = val;
    }
}
