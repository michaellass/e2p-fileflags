#[macro_use]
extern crate bitflags;

use e2p_sys::*;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:x}", Flags::NOATIME);
    }
}
