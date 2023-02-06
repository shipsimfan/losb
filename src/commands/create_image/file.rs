use super::error::CreateImageError;
use std::{
    io::{Seek, SeekFrom, Write},
    path::Path,
};

pub struct File(std::fs::File);

fn struct_to_slice<T: Sized>(value: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(value as *const T as *const u8, std::mem::size_of::<T>()) }
}

const MAX_ZERO_COUNT: usize = 1024 * 1024; // 1 MB

impl File {
    pub fn open_output<P: AsRef<Path>>(path: P) -> Result<Self, CreateImageError> {
        Ok(File(
            std::fs::OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(path)
                .map_err(|error| CreateImageError::WriteError(error))?,
        ))
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<(), CreateImageError> {
        self.0
            .write_all(buffer)
            .map_err(|error| CreateImageError::WriteError(error))
    }

    pub fn write_struct<T>(&mut self, data: &T) -> Result<(), CreateImageError> {
        self.write(struct_to_slice(data))
    }

    pub fn write_zeros(&mut self, count: usize) -> Result<(), CreateImageError> {
        if count > MAX_ZERO_COUNT {
            return self.chunked_write_zeros(count);
        }

        let buffer = vec![0; count];
        self.write(&buffer)
    }

    pub fn seek(&mut self, offset: usize) -> Result<(), CreateImageError> {
        self.0
            .seek(SeekFrom::Start(offset as u64))
            .map(|_| ())
            .map_err(|error| CreateImageError::WriteError(error))
    }

    pub fn set_len(&mut self, length: usize) -> Result<(), CreateImageError> {
        self.0
            .set_len(length as u64)
            .map_err(|error| CreateImageError::WriteError(error))
    }

    fn chunked_write_zeros(&mut self, count: usize) -> Result<(), CreateImageError> {
        let buffer = vec![0; MAX_ZERO_COUNT];
        for _ in 0..count / MAX_ZERO_COUNT {
            self.write(&buffer)?;
        }

        self.write(&buffer[..count % MAX_ZERO_COUNT])
    }
}
