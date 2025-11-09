use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
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

    pub fn replay<F: FnMut(u64, Operation, Vec<u8>, Vec<u8>)>(
        &mut self,
        mut apply: F,
    ) -> std::io::Result<()> {
        // todo write tests
        let mut hdr = [0u8; 17];
        loop {
            match self.file.read_exact(&mut hdr) {
                Ok(()) => {
                    let lsn = u64::from_le_bytes(hdr[0..8].try_into().unwrap());
                    let op = hdr[8];
                    let klen = u32::from_le_bytes(hdr[9..13].try_into().unwrap()) as usize;
                    let vlen = u32::from_le_bytes(hdr[13..17].try_into().unwrap()) as usize;
                    let mut k = vec![0; klen];
                    let mut v = vec![0; vlen];
                    if self.file.read_exact(&mut k).is_err() {
                        break;
                    }
                    if self.file.read_exact(&mut v).is_err() {
                        break;
                    }
                    apply(
                        lsn,
                        if op == 1 {
                            Operation::Insert
                        } else {
                            Operation::Delete
                        },
                        k,
                        v,
                    );
                }
                Err(_) => break,
            }
        }
        Ok(())
    }
}
