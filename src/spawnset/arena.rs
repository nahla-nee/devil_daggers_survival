use std::{io::{Read, Seek, Write}, mem::{size_of, align_of}};

#[cfg(feature = "json_coding")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "json_coding", derive(Serialize, Deserialize))]
pub struct Arena {
    pub arena: Vec<f32>
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            arena: vec![0f32; 51*51]
        }
    }

    pub fn size() -> usize {
        51*51*4
    }

    pub fn from_reader<R: Read + Seek>(reader: &mut R) -> Arena {
        let arena_size = 51*51;
        let arena_byte_size = arena_size*size_of::<f32>();
        let mut arena = vec![0.0f32; arena_size];

        unsafe {
            let mut arena_slice = std::slice::from_raw_parts_mut(arena.as_mut_ptr() as *mut u8,
            arena_byte_size);
            let _ = reader.read_exact(&mut arena_slice);
        };

        Arena {
            arena
        }
    }

    pub fn to_writer<W: Write>(&self, writer: &mut W) {
        let arena_size = 51*51;
        unsafe {
            let arena_slice = std::slice::from_raw_parts(self.arena.as_ptr() as *const u8,
            arena_size*size_of::<f32>());
            let _ = writer.write(&arena_slice);
        };
    }

    pub fn get_at(&self, x: usize, y: usize) -> f32 {
        self.arena[y*51+x]
    }

    pub fn set_at(&mut self, x: usize, y: usize, val: f32) {
        self.arena[y*51+x] = val;
    }
}
