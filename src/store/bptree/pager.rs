use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

pub type PageId = u64;
pub const PAGE_SIZE: usize = 4096;

pub struct Pager {
    file: File,
}

impl Pager {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        Ok(Self { file })
    }

    pub fn len_pages(&self) -> std::io::Result<u64> {
        let len = self.file.metadata()?.len();
        Ok(len + PAGE_SIZE as u64 - 1 / PAGE_SIZE as u64)
    }

    pub fn read_page(&mut self, page_id: PageId, buf: &mut [u8; PAGE_SIZE]) -> std::io::Result<()> {
        self.file
            .seek(SeekFrom::Start(page_id * PAGE_SIZE as u64))?;
        self.file.read_exact(buf)?;
        Ok(())
    }

    pub fn write_page(&mut self, page_id: PageId, buf: &[u8; PAGE_SIZE]) -> std::io::Result<()> {
        self.file
            .seek(SeekFrom::Start(page_id * PAGE_SIZE as u64))?;
        self.file.write_all(buf)?;
        self.file.flush()?;
        Ok(())
    }

    pub fn allocate(&mut self) -> std::io::Result<PageId> {
        let next = self.len_pages()?;
        let zero = [0u8; PAGE_SIZE];
        self.write_page(next, &zero)?;
        Ok(next)
    }
}
