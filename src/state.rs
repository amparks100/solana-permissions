use solana_program::{
    program_pack::{IsInitialized, Sealed},
};

use crate::{util::Serdes};
use borsh::{BorshDeserialize, BorshSerialize};

/**
 * State for main program node
 */
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct PermissionState {
    pub is_initialized: bool,
    pub permissions: u32,
    pub role: u32,
}

impl Sealed for PermissionState {}
impl Serdes for PermissionState {}

impl IsInitialized for PermissionState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

// enum Permission {
// 	/**
// 	 * @dev 0x0 NONE reserved for no permissions
// 	 */
// 	NONE,
// 	/**
// 	 * @dev 0x1 Sign a DSNP Announcement
// 	 */
// 	ANNOUNCE,
// 	/**
// 	 * @dev 0x2 Add new delegate
// 	 */
// 	OWNERSHIP_TRANSFER,
// 	/**
// 	 * @dev 0x3 Add new delegates
// 	 */
// 	DELEGATE_ADD,
// 	/**
// 	 * @dev 0x4 Remove delegates
// 	 */
// 	DELEGATE_REMOVE
// }
// pub enum Permission {
//     None = 0x0,
//     Announce = 0x1,
//     OwnershipTransfer = 0x2,
//     DelegateAdd = 0x3,
//     DelegateRemove = 0x4,
// }

// enum Role {
// 	/**
// 	 * @dev 0x0 NONE reserved for no permissions
// 	 */
// 	NONE,
// 	/**
// 	 * @dev 0x1 OWNER:
// 	 *      - Permission.*
// 	 */
// 	OWNER,
// 	/**
// 	 * @dev 0x2 ANNOUNCER:
// 	 *      - Permission.ANNOUNCE
// 	 */
// 	ANNOUNCER
// }

// pub enum Role {
//     None = 0x0,
//     Owner = 0x1,
//     Announcer = 0x2,
// }