use solana_program::{program_error::ProgramError, msg};

use borsh::{BorshDeserialize, BorshSerialize};

pub trait Serdes: Sized + BorshSerialize + BorshDeserialize {
	fn pack(&self, dst: &mut [u8]) {
		let encoded = self.try_to_vec().unwrap();
		dst[..encoded.len()].copy_from_slice(&encoded);
	}
	fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
		Self::try_from_slice(src).map_err(|_| ProgramError::InvalidAccountData)
	}
}

pub const PERMISSION_NONE: u8 = 0x1;
pub const PERMISSION_PUBLISH: u8 = 0x2;
pub const PERMISSION_OWNERSHIPTRANSFER: u8 = 0x3;
pub const PERMISSION_DELEGATEADD: u8 = 0x4;
pub const PERMISSION_DELEGATEREMOVE: u8 = 0x5;

//pub const ROLE_NONE: u32 = 0x0;
pub const ROLE_OWNER: u8 = 0;
pub const ROLE_PUBLISH: u8 = 1;


// uint256 private constant ROLE_PERMISSIONS =
//         // Role.OWNER Mask
        // (((1 << uint32(Permission.ANNOUNCE)) |
        //     (1 << uint32(Permission.OWNERSHIP_TRANSFER)) |
        //     (1 << uint32(Permission.DELEGATE_ADD)) |
        //     (1 << uint32(Permission.DELEGATE_REMOVE))) << (uint32(Role.OWNER) << 5)) |
        //     // Role.ANNOUNCER Mask
        //     ((1 << uint32(Permission.ANNOUNCE)) << (uint32(Role.ANNOUNCER) << 5));

static ROLE_OWNER_BIT: u64 = ((1 << PERMISSION_PUBLISH) 
| (1 << PERMISSION_OWNERSHIPTRANSFER) 
| (1 << PERMISSION_DELEGATEADD) 
| (1 << PERMISSION_DELEGATEREMOVE)) 
<< (ROLE_OWNER << 5);


static ROLE_ANNOUNCER_BIT: u64 = (1 << PERMISSION_PUBLISH) << (ROLE_PUBLISH << 5) ;

static ROLE_PERMISSIONS: u64 = ROLE_OWNER_BIT | ROLE_ANNOUNCER_BIT; 


// /**
//      * @dev Check to see if the role has a particular permission
//      * @param role The Role to test against
//      * @param permission The Permission to test with the role
//      * @return true if the role is assigned the given permission
//      */
//     function doesRoleHavePermission(Role role, Permission permission) public pure returns (bool) {
//         // bitwise (possible) AND (check single permission mask)
//        return ROLE_PERMISSIONS & (((1 << uint32(permission))) << (uint32(role) << 5)) > 0x0;
//     }
pub fn does_role_have_permission(role: u8, permission: u8) -> bool {
	msg!("RolePermissions {}, RoleOwner {}, RoleAnnouncer {}", ROLE_PERMISSIONS, ROLE_OWNER_BIT, ROLE_ANNOUNCER_BIT );
	msg!("Params {} {}", role, permission);
	return ROLE_PERMISSIONS & ((1 << permission) << (role << 5)) > 0x0;
}