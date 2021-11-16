use std::{io::{Write, Read, Seek, SeekFrom}, mem::size_of};
use super::utils::*;


pub fn size(world_version: i32) -> usize {
    if world_version == 8 {
        36
    }
    else {
        40
    }
}

pub fn from_reader<R: Read + Seek>(world_version: i32, reader: &mut R) -> i32 {
    // dump initial unknown/useless values
    let skip_byte_count = (size(world_version)-size_of::<i32>()).try_into().unwrap(); // shouldn't ever fail
    let _ = reader.seek(SeekFrom::Current(skip_byte_count));

    // Spawns count
    get_i32(reader)
}

pub fn to_writer<W: Write>(spawns_count: i32, world_version: i32, writer: &mut W) {
    let _ = writer.write(&0u32.to_le_bytes());
    let _ = writer.write(&0u32.to_le_bytes());
    let _ = writer.write(&0u32.to_le_bytes());
    let _ = writer.write(&0x00000001u32.to_le_bytes());
    let _ = writer.write(&0x000001F4u32.to_le_bytes());
    let _ = writer.write(&0x000000FAu32.to_le_bytes());
    let _ = writer.write(&0x00000078u32.to_le_bytes());
    let _ = writer.write(&0x0000003Cu32.to_le_bytes());

    if world_version != 8 {
        let _ = writer.write(&0u32.to_le_bytes());
    }

    let _ = writer.write(&spawns_count.to_le_bytes());
}