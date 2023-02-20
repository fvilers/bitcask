use std::io;

use crc::{Crc, CRC_32_ISCSI};

use crate::timestamp::timestamp_secs;

pub struct DatastoreEntry {
    crc: u32,
    pub timestamp: u64,
    key_size: u32,
    pub value_size: u32,
    key: String,
    value: Vec<u8>,
}

const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

fn compute_crc(value: &[u8]) -> u32 {
    let mut crc = CASTAGNOLI.digest();
    crc.update(value);
    crc.finalize()
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
