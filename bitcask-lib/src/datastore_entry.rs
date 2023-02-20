use std::{io, str};

use byteorder::{BigEndian, ReadBytesExt};
use crc::{Crc, CRC_32_ISCSI};

use crate::{datastore::Result, datastore_error::DatastoreError, timestamp::timestamp_secs};

pub struct DatastoreEntry {
    crc: u32,
    pub timestamp: u64,
    key_size: u32,
    pub value_size: u32,
    pub key: String,
    pub value: Vec<u8>,
}

impl DatastoreEntry {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        let crc = compute_crc(value.as_slice());
        let timestamp = timestamp_secs();
        let key_size: u32 = key.len().try_into().unwrap();
        let value_size: u32 = value.len().try_into().unwrap();

        Self {
            crc,
            timestamp,
            key_size,
            value_size,
            key,
            value,
        }
    }

    pub fn read(file: &mut dyn io::Read) -> Result<Self> {
        let crc = file.read_u32::<BigEndian>()?;
        let timestamp = file.read_u64::<BigEndian>()?;
        let key_size = file.read_u32::<BigEndian>()?;
        let value_size = file.read_u32::<BigEndian>()?;

        let mut key: Vec<u8> = vec![0u8; key_size as usize];
        file.read_exact(&mut key)?;
        let key = str::from_utf8(&key)?;

        let mut value: Vec<u8> = vec![0u8; value_size as usize];
        file.read_exact(&mut value)?;

        if crc != compute_crc(&value) {
            return Err(DatastoreError::CrcMismatch);
        }

        Ok(DatastoreEntry {
            crc,
            timestamp,
            key_size,
            value_size,
            key: key.to_owned(),
            value: value.to_owned(),
        })
    }

    pub fn write(&self, file: &mut dyn io::Write) -> io::Result<()> {
        file.write_all(&self.crc.to_be_bytes())?;
        file.write_all(&self.timestamp.to_be_bytes())?;
        file.write_all(&self.key_size.to_be_bytes())?;
        file.write_all(&self.value_size.to_be_bytes())?;
        file.write_all(self.key.as_bytes())?;
        file.write_all(&self.value)?;

        Ok(())
    }
}

const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

fn compute_crc(value: &[u8]) -> u32 {
    let mut crc = CASTAGNOLI.digest();
    crc.update(value);
    crc.finalize()
}
