use std::io::Read;

pub fn get_i8<R: Read>(reader: &mut R) -> i8 {
    let mut tmp_buffer: [u8; 1] = [0; 1];
    let _ = reader.read(&mut tmp_buffer);
    i8::from_le_bytes(tmp_buffer)
}

pub fn get_i32<R: Read>(reader: &mut R) -> i32 {
    let mut tmp_buffer: [u8; 4] = [0; 4];
    let _ = reader.read(&mut tmp_buffer);
    i32::from_le_bytes(tmp_buffer)
}

pub fn get_f32<R: Read>(reader: &mut R) -> f32 {
    let mut tmp_buffer: [u8; 4] = [0; 4];
    let _ = reader.read(&mut tmp_buffer);
    f32::from_le_bytes(tmp_buffer)
}