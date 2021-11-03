use thiserror::Error;

#[derive(Error, Debug)]
pub enum DDError{
	#[error("Not enough data to read")]
	NotEnoughDataRead,
	#[error("Not enough space to write data to buffer")]
	NotEnoughDataWrite,
	#[error("Failed to read data from disk\n\tCaused by: {0}")]
	IORead(std::io::Error),
	#[error("Failed to write data to disk\n\tCaused by: {0}")]
	IOWrite(std::io::Error)
}