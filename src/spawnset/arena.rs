use crate::byte_reader::ByteReader;

pub struct Arena {
    arena: [f32; 2601]
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            arena: [0f32; 2601]
        }
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader) -> Arena {
        let mut arena = [0f32; 2601];
        unsafe {
            byte_reader.get_chunk(arena.as_mut_ptr(), 2601)
                .expect("Failed to read arena array, not enough data");
        }

        Arena {
            arena
        }
    }

    pub fn get_at(&self, x: usize, y: usize) -> f32 {
        self.arena[y*51+x]
    }

    pub fn set_at(&mut self, x: usize, y: usize, val: f32) {
        self.arena[y*51+x] = val;
    }
}
