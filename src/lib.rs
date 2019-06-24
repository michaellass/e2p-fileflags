/*
MIT License

Copyright (c) 2019 Michael Lass

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#[macro_use]
extern crate bitflags;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use e2p_sys::*;
use std::ffi::CString;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::os::unix::io::AsRawFd;
use std::path::Path;

bitflags! {
    #[derive(Default)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Flags: u32 {
        const SECRM = EXT2_SECRM_FL;
        const UNRM = EXT2_UNRM_FL;
        const COMPR = EXT2_COMPR_FL;
        const SYNC = EXT2_SYNC_FL;
        const IMMUTABLE = EXT2_IMMUTABLE_FL;
        const APPEND = EXT2_APPEND_FL;
        const NODUMP = EXT2_NODUMP_FL;
        const NOATIME = EXT2_NOATIME_FL;
        const DIRTY = EXT2_DIRTY_FL;
        const COMPRBLK = EXT2_COMPRBLK_FL;
        const NOCOMPR = EXT2_NOCOMPR_FL;

        #[cfg(ENCRYPT)]
        const ENCRYPT = EXT4_ENCRYPT_FL;

        const BTREE = EXT2_BTREE_FL;
        const INDEX = EXT2_INDEX_FL;
        const IMAGIC = EXT2_IMAGIC_FL;
        const JOURNAL_DATA = EXT3_JOURNAL_DATA_FL;
        const NOTAIL = EXT2_NOTAIL_FL;
        const DIRSYNC = EXT2_DIRSYNC_FL;
        const TOPDIR = EXT2_TOPDIR_FL;
        const HUGE_FILE = EXT4_HUGE_FILE_FL;
        const EXTENTS = EXT4_EXTENTS_FL;

        #[cfg(VERITY)]
        const VERITY = EXT4_VERITY_FL;

        const EA_INODE = EXT4_EA_INODE_FL;
        const NOCOW = FS_NOCOW_FL;
        const SNAPFILE = EXT4_SNAPFILE_FL;
        const SNAPFILE_DELETED = EXT4_SNAPFILE_DELETED_FL;
        const SNAPFILE_SHRUNK = EXT4_SNAPFILE_SHRUNK_FL;

        #[cfg(INLINE_DATA)]
        const INLINE_DATA = EXT4_INLINE_DATA_FL;

        #[cfg(PROJINHERIT)]
        const PROJINHERIT = EXT4_PROJINHERIT_FL;

        #[cfg(CASEFOLD)]
        const CASEFOLD = EXT4_CASEFOLD_FL;

        const RESERVED = EXT2_RESERVED_FL;
        const USER_VISIBLE = EXT2_FL_USER_VISIBLE;
        const USER_MODIFIABLE = EXT2_FL_USER_MODIFIABLE;
    }
}

pub trait FileFlags {
    fn flags(&self) -> Result<Flags, Error>;
    fn set_flags(&self, f: impl AsRef<Flags>) -> Result<(), Error>;
}

impl AsRef<Flags> for Flags {
    fn as_ref(&self) -> &Flags {
        &self
    }
}

impl FileFlags for Path {
    fn flags(&self) -> Result<Flags, Error> {
        let path_cstr = match self.to_str() {
            Some(s) => CString::new(s)?,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Provided path is no valid Unicode",
                ));
            }
        };
        let ret: i32;
        let mut retflags: u64 = 0;
        let path_ptr = path_cstr.as_ptr();
        let retflags_ptr: *mut u64 = &mut retflags;

        unsafe {
            ret = fgetflags(path_ptr, retflags_ptr);
        }

        match ret {
            0 => match Flags::from_bits(retflags as u32) {
                Some(f) => Ok(f),
                None => Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unexcpected flags encountered",
                )),
            },
            _ => Err(Error::last_os_error()),
        }
    }

    fn set_flags(&self, f: impl AsRef<Flags>) -> Result<(), Error> {
        let path_cstr = match self.to_str() {
            Some(s) => CString::new(s)?,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Provided path is no valid Unicode",
                ));
            }
        };
        let ret: i32;
        let intflags: u64 = f.as_ref().bits() as u64;
        let path_ptr = path_cstr.as_ptr();

        unsafe {
            ret = fsetflags(path_ptr, intflags);
        }

        match ret {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

impl FileFlags for File {
    fn flags(&self) -> Result<Flags, Error> {
        let ret: i32;
        let mut retflags: u64 = 0;
        let retflags_ptr: *mut u64 = &mut retflags;

        unsafe {
            ret = getflags(self.as_raw_fd(), retflags_ptr);
        }

        match ret {
            0 => match Flags::from_bits(retflags as u32) {
                Some(f) => Ok(f),
                None => Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unexcpected flags encountered",
                )),
            },
            _ => Err(Error::last_os_error()),
        }
    }

    fn set_flags(&self, f: impl AsRef<Flags>) -> Result<(), Error> {
        let ret: i32;
        let intflags: u64 = f.as_ref().bits() as u64;

        unsafe {
            ret = setflags(self.as_raw_fd(), intflags);
        }

        match ret {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{remove_file, File};

    /* This test is currently known to fail on filesystems different to ext4:
     * XFS does not support the NOCOW flag
     * F2FS implicitly sets the INLINE_DATA flag on the test file */
    #[test]
    fn unified() {
        let mut p = env::current_dir().unwrap();
        p.push("e2p-fileflags-testfile-voo4JooY");
        let f = File::create(&p).unwrap();

        assert_eq!(f.flags().unwrap(), Flags::empty());
        p.set_flags(Flags::NOCOW).unwrap();
        assert_eq!(f.flags().unwrap(), Flags::NOCOW);
        p.set_flags(Flags::NOCOW | Flags::NOATIME).unwrap();
        assert_eq!(f.flags().unwrap(), Flags::NOATIME | Flags::NOCOW);
        p.set_flags(&Flags::NOCOW).unwrap();
        assert_eq!(f.flags().unwrap(), Flags::NOCOW);
        p.set_flags(&Flags::empty()).unwrap();
        assert_eq!(f.flags().unwrap(), Flags::empty());

        assert_eq!(p.flags().unwrap(), Flags::empty());
        f.set_flags(Flags::NOCOW).unwrap();
        assert_eq!(p.flags().unwrap(), Flags::NOCOW);
        f.set_flags(Flags::NOCOW | Flags::NOATIME).unwrap();
        assert_eq!(p.flags().unwrap(), Flags::NOATIME | Flags::NOCOW);
        f.set_flags(&Flags::NOCOW).unwrap();
        assert_eq!(p.flags().unwrap(), Flags::NOCOW);
        f.set_flags(&Flags::empty()).unwrap();
        assert_eq!(p.flags().unwrap(), Flags::empty());

        drop(f);
        let _ = remove_file(p);
    }
}
