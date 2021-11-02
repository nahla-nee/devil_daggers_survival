use thiserror::Error;

#[derive(Error, Debug)]
pub enum ByteReaderError {
    #[error("There's not enough data in the buffer to read the desired type")]
    NotEnoughData
}