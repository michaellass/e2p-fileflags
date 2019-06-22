#[macro_use]
extern crate bitflags;

use e2p_sys::*;
use nix;
use std::ffi::CString;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::Path;

bitflags! {
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
        const VERITY = EXT4_VERITY_FL;
        const EA_INODE = EXT4_EA_INODE_FL;
        const NOCOW = FS_NOCOW_FL;
        const SNAPFILE = EXT4_SNAPFILE_FL;
        const SNAPFILE_DELETED = EXT4_SNAPFILE_DELETED_FL;
        const SNAPFILE_SHRUNK = EXT4_SNAPFILE_SHRUNK_FL;
        const INLINE_DATA = EXT4_INLINE_DATA_FL;
        const PROJINHERIT = EXT4_PROJINHERIT_FL;
        const CASEFOLD = EXT4_CASEFOLD_FL;
        const RESERVED = EXT2_RESERVED_FL;
        const USER_VISIBLE = EXT2_FL_USER_VISIBLE;
        const USER_MODIFIABLE = EXT2_FL_USER_MODIFIABLE;
    }
}

pub trait FileFlags {
    fn flags(&self) -> Result<Flags, nix::Error>;
    fn set_flags(&self, f: impl AsRef<Flags>) -> Result<(), nix::Error>;
    // fn add_flags(&self, f: impl AsRef<Flags>) -> Result<(), nix::Error>;
    // fn remove_flags(&self, f: impl AsRef<Flags>) -> Result<(), nix::Error>;
}

impl AsRef<Flags> for Flags {
    fn as_ref(&self) -> &Flags {
        &self
    }
}

impl FileFlags for Path {
    fn flags(&self) -> Result<Flags, nix::Error> {
        let path_cstr = CString::new(self.to_str().expect("Could not convert Path to str"))
            .expect("Could not convert str to CStr");
        let ret: i32;
        let mut retflags: u64 = 0;
        let path_ptr = path_cstr.as_ptr();
        let retflags_ptr: *mut u64 = &mut retflags;

        unsafe {
            ret = fgetflags(path_ptr, retflags_ptr);
        }

        match ret {
            0 => Ok(Flags::from_bits(retflags as u32)
                .expect("Failed to interpret return value as fileflag")),
            _ => Err(nix::Error::last()),
        }
    }

    fn set_flags(&self, f: impl AsRef<Flags>) -> Result<(), nix::Error> {
        let path_cstr = CString::new(self.to_str().expect("Could not convert Path to str"))
            .expect("Could not convert str to CStr");
        let ret: i32;
        let intflags: u64 = f.as_ref().bits() as u64;
        let path_ptr = path_cstr.as_ptr();

        unsafe {
            ret = fsetflags(path_ptr, intflags);
        }

        match ret {
            0 => Ok(()),
            _ => Err(nix::Error::last()),
        }
    }
}

impl FileFlags for File {
    fn flags(&self) -> Result<Flags, nix::Error> {
        let ret: i32;
        let mut retflags: u64 = 0;
        let retflags_ptr: *mut u64 = &mut retflags;

        unsafe {
            ret = getflags(self.as_raw_fd(), retflags_ptr);
        }

        match ret {
            0 => Ok(Flags::from_bits(retflags as u32)
                .expect("Failed to interpret return value as fileflag")),
            _ => Err(nix::Error::last()),
        }
    }

    fn set_flags(&self, f: impl AsRef<Flags>) -> Result<(), nix::Error> {
        let ret: i32;
        let intflags: u64 = f.as_ref().bits() as u64;

        unsafe {
            ret = setflags(self.as_raw_fd(), intflags);
        }

        match ret {
            0 => Ok(()),
            _ => Err(nix::Error::last()),
        }
    }
}
