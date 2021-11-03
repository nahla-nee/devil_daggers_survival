use std::intrinsics::copy_nonoverlapping;
use std::mem::size_of;
use crate::dd_error::DDError;

/// A helper utility to make reading data from a byte buffer easier
pub struct ByteReader {
    bytes: Vec<u8>,
    position: usize
}

impl ByteReader {
    pub fn new(bytes: Vec<u8>) -> ByteReader {
        ByteReader {
            bytes,
            position: 0
        }
    }

    pub fn bytes_left(&self) -> usize {
        self.bytes.len() - self.position
    }

    pub fn skip_bytes(&mut self, bytes: usize) {
        if bytes > self.bytes_left() {
            self.position = self.bytes.len();
        }
        else{
            self.position = self.position+bytes;
        }
    }

    pub fn get_u8(&mut self) -> Result<u8, DDError> {
        if size_of::<u8>() > self.bytes_left() {
            return Err(DDError::NotEnoughData)
        }

        let val = Ok(self.bytes[self.position]);
        self.position = self.position + size_of::<u8>();
        val
    }

    pub fn get_u32(&mut self) -> Result<u32, DDError> {
        if size_of::<u32>() > self.bytes_left() {
            return Err(DDError::NotEnoughData)
        }

        let bytes = &self.bytes[self.position..self.position+size_of::<u32>()];
        let val = Ok(u32::from_le_bytes(bytes.try_into().unwrap()));
        self.position = self.position + size_of::<u32>();
        val
    }

    pub fn get_i32(&mut self) -> Result<i32, DDError> {
        if size_of::<i32>() > self.bytes_left() {
            return Err(DDError::NotEnoughData)
        }

        let bytes = &self.bytes[self.position..self.position+size_of::<i32>()];
        let val = Ok(i32::from_le_bytes(bytes.try_into().unwrap()));
        self.position = self.position + size_of::<i32>();
        val
    }

    pub fn get_f32(&mut self) -> Result<f32, DDError> {
        if size_of::<f32>() > self.bytes_left() {
            return Err(DDError::NotEnoughData)
        }

        let bytes = &self.bytes[self.position..self.position+size_of::<f32>()];
        let val = Ok(f32::from_le_bytes(bytes.try_into().unwrap()));
        self.position = self.position + size_of::<f32>();
        val
    }

    pub unsafe fn get_chunk<T>(&mut self, ptr: *mut T, count: usize) -> Result<(), DDError> {
        if size_of::<T>()*count > self.bytes_left() {
            return Err(DDError::NotEnoughData)
        }

        copy_nonoverlapping(self.bytes.as_ptr().offset(
            self.position.try_into().unwrap()) as _, ptr, count);
        self.position = self.position+size_of::<T>()*count;

        Ok(())
    }
}
