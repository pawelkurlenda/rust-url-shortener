use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

#[derive(Clone, Copy)]
pub enum Operation {
    Insert,
    Delete,
}

/// Write-Ahead Log structure for B+ Tree operations
pub struct Wal {
    file: File,
    /// Next Log Sequence Number
    pub next_lsn: u64,
}

impl Wal {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;
        Ok(Self { file, next_lsn: 1 })
    }

    pub fn append(
        &mut self,
        operation: Operation,
        key: &[u8],
        value: &[u8],
    ) -> std::io::Result<u64> {
        let lsn = self.next_lsn;
        self.next_lsn += 1;
        self.file.write_all(&lsn.to_le_bytes())?;
        self.file.write_all(&[operation as u8])?;
        self.file.write_all(&(key.len() as u32).to_le_bytes())?; // todo check key max length
        self.file.write_all(&(value.len() as u32).to_le_bytes())?;
        self.file.write_all(key)?;
        self.file.write_all(value)?;
        self.file.flush()?;
        Ok(lsn)
    }

    pub fn replay(&self) -> std::io::Result<()> {
        unimplemented!(); // todo Implement log replay logic here
    }
}
