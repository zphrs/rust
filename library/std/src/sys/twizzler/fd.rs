use crate::io::{self, Read, SeekFrom, Error, ErrorKind};
use crate::io::SeekFrom::{Start, Current, End};

use crate::sys::unsupported;
use twizzler_runtime_api::RawFd;
use crate::sys_common::IntoInner;
use crate::sys_common::FromInner;

use twizzler_runtime_api::SeekFrom as InnerSeek;
use twizzler_runtime_api::SeekFrom::{Start as InnerStart, Current as InnerCurrent, End as InnerEnd};
use twizzler_runtime_api::FsError;

#[stable(feature = "FsError Conversion", since = "0.1.0")]
impl core::convert::From<FsError> for io::Error {
    fn from(error: FsError) -> io::Error {
        match error {
            FsError::Other => io::Error::new(io::ErrorKind::Other, "unknown error"),
            FsError::InvalidPath => io::Error::from(ErrorKind::NotFound),
            FsError::LookupError => io::Error::new(ErrorKind::Other, "File Descriptor not found"),
            FsError::SeekError => io::Error::from(ErrorKind::UnexpectedEof),
        }
    }
}

// A abstraction that can do continious IO on a set of Twizzler objects
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct FileDesc {
    pub fd: RawFd
}

impl FileDesc {
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let runtime = twizzler_runtime_api::get_runtime();

        let result = runtime.read(self.fd, buf)?;

        Ok(result as usize)
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut me = self;
        (&mut me).read_to_end(buf)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let runtime = twizzler_runtime_api::get_runtime();

        let result = runtime.write(self.fd, buf)?;
        
        Ok(result as usize)
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let runtime = twizzler_runtime_api::get_runtime();

        let inner: InnerSeek = match pos {
            Start(x) => InnerSeek::Start(x),
            End(x) => InnerSeek::End(x),
            Current(x) => InnerSeek::Current(x),
        };

        let result = runtime.seek(self.fd, inner)?;

        Ok(result as u64)
    }

    pub fn duplicate(&self) -> io::Result<FileDesc> {
        self.duplicate_path(&[])
    }

    pub fn duplicate_path(&self, _path: &[u8]) -> io::Result<FileDesc> {
        unsupported()
    }

    pub fn nonblocking(&self) -> io::Result<bool> {
        Ok(false)
    }

    pub fn set_cloexec(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
        unsupported()
    }
}

impl IntoInner<RawFd> for FileDesc {
    fn into_inner(self) -> RawFd {
        self.fd
    }
}

impl FromInner<RawFd> for FileDesc {
    fn from_inner(owned_fd: RawFd) -> Self {
        Self { fd: owned_fd }
    }
}

impl core::fmt::Display for FileDesc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "FileDesc({:x})", self.fd)
    }
}