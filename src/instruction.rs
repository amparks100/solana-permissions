use crate::{util::Serdes};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum PermissionInstruction {
    InitPermission,
    UpdatePermission,
}
impl Serdes for PermissionInstruction {}

/// Initializes a new Permission with a state account.
///
/// Accounts expected:
///
/// 0. `[signer]` The account of the person who needs permissions
/// 1. `[writable]` Account to hold Permission state data (33 bytes)
/// 2. `[]` The rent sysvar
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct InitPermission { 
    kind: PermissionInstruction
}

impl Serdes for InitPermission {}

/// Updates a value in the Permission state
///
/// Accounts expected:
///
/// 0. `[signer]` The account of the person initializing the escrow
/// 1. `[]` Permission State account
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct UpdatePermission {
    pub kind: PermissionInstruction,
    pub role: u8,
}

impl Serdes for UpdatePermission {}
