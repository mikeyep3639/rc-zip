use super::encoding;
use std::{error, fmt};

/// Any zip-related error, from invalid archives to encoding problems.
#[derive(Debug)]
pub enum Error {
    /// Not a valid zip file, or a variant that is unsupported.
    Format(FormatError),
    /// Invalid UTF-8, Shift-JIS, or any problem encountered while decoding text in general.
    Encoding(encoding::DecodingError),
    /// I/O-related error
    ///
    /// Only returned by the higher-level API, since
    /// [ArchiveReader](crate::reader::ArchiveReader) lets you do your own I/O.
    IO(std::io::Error),
    /// Could not read as a zip because size could not be determined
    UnknownSize,
}

/// Specific zip format errors, mostly due to invalid zip archives but that could also stem from
/// implementation shortcomings.
#[derive(Debug)]
pub enum FormatError {
    /// The end of central directory record was not found.
    ///
    /// This usually indicates that the file being read is not a zip archive.
    DirectoryEndSignatureNotFound,
    /// The zip64 end of central directory record could not be parsed.
    ///
    /// This is only returned when a zip64 end of central directory *locator* was found,
    /// so the archive should be zip64, but isn't.
    Directory64EndRecordInvalid,
    /// Corrupted/partial zip file: the offset we found for the central directory
    /// points outside of the current file.
    DirectoryOffsetPointsOutsideFile,
    /// The central record is corrupted somewhat.
    ///
    /// This can happen when the end of central directory record advertises
    /// a certain number of files, but we weren't able to read the same number of central directory
    /// headers.
    InvalidCentralRecord,
    /// An extra field (that we support) was not decoded correctly.
    ///
    /// This can indicate an invalid zip archive, or an implementation error in this crate.
    InvalidExtraField,
    /// End of central directory record claims an impossible number of file.
    ///
    /// Each entry takes a minimum amount of size, so if the overall archive size is smaller than
    /// claimed_records_count * minimum_entry_size, we know it's not a valid zip file.
    ImpossibleNumberOfFiles {
        claimed_records_count: u64,
        zip_size: u64,
    },
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(e) => write!(f, "rc-zip: {}", e),
            Error::Encoding(e) => write!(f, "rc-zip: {:#?}", e),
            Error::Format(e) => write!(f, "rc-zip: {:#?}", e),
            Error::UnknownSize => write!(f, "rc-zip: file size must be known to open zip archive",),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<FormatError> for Error {
    fn from(e: FormatError) -> Self {
        Error::Format(e)
    }
}

impl From<encoding::DecodingError> for Error {
    fn from(e: encoding::DecodingError) -> Self {
        Error::Encoding(e)
    }
}

/// Parsing error, see [ZipParseResult].
pub type ZipParseError<'a> = (&'a [u8], nom::error::ErrorKind);

/// Result of a parse operation
///
/// Used internally when parsing, for example, the end of central directory record.
pub type ZipParseResult<'a, T> = nom::IResult<&'a [u8], T, ZipParseError<'a>>;
