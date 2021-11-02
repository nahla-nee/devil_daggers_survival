use crate::byte_reader::ByteReader;

pub struct Settings {
    initial_hand: u8,
    additional_gems: u32,
    time_start: Option<f32>
}

impl Settings {
    pub fn new(initial_hand: u8, additional_gems: u32, time_start: Option<f32>) -> Settings {
        Settings {
            initial_hand,
            additional_gems,
            time_start
        }
    }

    pub fn from_byte_reader(byte_reader: &mut ByteReader, spawn_ver: u32) -> Settings {
        if byte_reader.bytes_left() < 9 {
        }

        let initial_hand = byte_reader.get_u8().unwrap();
        let additional_gems = byte_reader.get_u32().unwrap();
        let mut time_start = None;
        if spawn_ver > 5 {
            time_start = Some(byte_reader.get_f32().unwrap());
        }

        Self::new(initial_hand, additional_gems, time_start)
    }

    pub fn get_initial_hand(&self) -> u8 {
        self.initial_hand
    }

    pub fn get_additional_gems(&self) -> u32 {
        self.additional_gems
    }

    pub fn get_time_start(&self) -> Option<f32> {
        self.time_start
    }

    pub fn set_initial_hand(&mut self, val: u8) {
        self.initial_hand = val
    }

    pub fn set_additional_gems(&mut self, val: u32) {
        self.additional_gems = val
    }

    pub fn set_time_start(&mut self, val: Option<f32>) {
        self.time_start = val
    }
}