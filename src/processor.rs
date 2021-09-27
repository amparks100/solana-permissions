use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_pack::{IsInitialized},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    msg,
};

use crate::{
    error::PermissionError,
    instruction::{ UpdatePermission },
    util::{Serdes, does_role_have_permission},
    state::{ PermissionState }
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction_type = instruction_data[0];

        if instruction_type == 0 {
            return Self::process_init_permission(accounts, program_id)
        }
        
        else if instruction_type == 1 {
            let instruction = UpdatePermission::unpack(instruction_data)?;
            return Self::process_set_value(accounts, instruction.permission, instruction.role, program_id)
        }

        Err(PermissionError::InvalidInstruction.into())
    }

    fn process_init_permission(
        accounts: &[AccountInfo],
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let signer = next_account_info(account_info_iter)?;
        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let permission_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(permission_account.lamports(), permission_account.data_len()) {
            return Err(PermissionError::NotRentExempt.into());
        }

        // initialize permissions
        let mut permission_info = PermissionState::unpack(&permission_account.data.borrow())?;
        if permission_info.is_initialized()  {
            return Err(PermissionError::InvalidInstruction.into());
        }
        permission_info.is_initialized = true;
        permission_info.permissions = 0;
        permission_info.role = 0;

        if !rent.is_exempt(permission_account.lamports(), permission_account.data_len()) {
            return Err(PermissionError::NotRentExempt.into());
        }

        PermissionState::pack(&permission_info, &mut permission_account.data.borrow_mut());

        Ok(())
    }

    fn process_set_value(
        accounts: &[AccountInfo],
        permission: u32,
        role: u32,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let signer = next_account_info(account_info_iter)?;
        //verify signer
        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let permission_account = next_account_info(account_info_iter)?;
        let _rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        // State must already be initialized
        let mut permission_info = PermissionState::unpack(&permission_account.data.borrow())?;
        if !permission_info.is_initialized() {
            return Err(PermissionError::InvalidInstruction.into());
        }

        //verify enums are valid
        if !does_role_have_permission(role, permission){
            return Err(PermissionError::InvalidPermission);
        }

        //update permission enums
        permission_info.permissions = permission;
        permission_info.role = role;
        PermissionState::pack(&permission_info, &mut permission_account.data.borrow_mut());
        
        Ok(())
    }
}
