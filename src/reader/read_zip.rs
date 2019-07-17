use crate::{
    error::Error,
    reader::{ArchiveReader, ArchiveReaderResult},
    types::Archive,
};
use positioned_io::{Cursor, ReadAt};

/// A trait for reading something as a zip archive (blocking I/O model)
pub trait ReadZipWithSize {
    /// Reads self as a zip archive.
    ///
    /// This functions blocks until the entire archive has been read.
    /// It is not compatible with non-blocking or async I/O.
    fn read_zip_with_size(&self, size: u64) -> Result<Archive, Error>;
}

/// A trait for reading something as a zip archive (blocking I/O model),
/// when we can tell size from self.
pub trait ReadZip {
    /// Reads self as a zip archive.
    ///
    /// This functions blocks until the entire archive has been read.
    /// It is not compatible with non-blocking or async I/O.
    fn read_zip(&self) -> Result<Archive, Error>;
}

impl ReadZipWithSize for ReadAt {
    fn read_zip_with_size(&self, size: u64) -> Result<Archive, Error> {
        let mut ar = ArchiveReader::new(size);
        loop {
            if let Some(offset) = ar.wants_read() {
                match ar.read(&mut Cursor::new_pos(&self, offset)) {
                    Ok(read_bytes) => {
                        if read_bytes == 0 {
                            return Err(Error::IO(std::io::ErrorKind::UnexpectedEof.into()));
                        }
                    }
                    Err(err) => return Err(Error::IO(err)),
                }
            }

            match ar.process()? {
                ArchiveReaderResult::Done(archive) => return Ok(archive),
                ArchiveReaderResult::Continue => {}
            }
        }
    }
}

impl ReadZip for Vec<u8> {
    fn read_zip(&self) -> Result<Archive, Error> {
        (self as &ReadAt).read_zip_with_size(self.len() as u64)
    }
}

