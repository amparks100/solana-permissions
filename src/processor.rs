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
    util::{Serdes, does_role_have_permission, PERMISSION_DELEGATEADD, ROLE_OWNER},
    state::{ PermissionState }
};

use std::mem;


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
            return Self::process_set_value(accounts, instruction.role, program_id)
        }

        Err(PermissionError::InvalidInstruction.into())
    }

    fn process_init_permission(
        accounts: &[AccountInfo],
        _program_id: &Pubkey,
    ) -> ProgramResult {
        msg!("Entered Init Permission");
        let account_info_iter = &mut accounts.iter();

        let signer = next_account_info(account_info_iter)?;
        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let permission_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        msg!("got accoutns");
        if !rent.is_exempt(permission_account.lamports(), permission_account.data_len()) {
            return Err(PermissionError::NotRentExempt.into());
        }

        // initialize permissions
        msg!("Struct size{}", mem::size_of::<PermissionState>());
        let mut permission_info = PermissionState::unpack(&permission_account.data.borrow())?;
        msg!("unpacked");
        if permission_info.is_initialized()  {
            return Err(PermissionError::InvalidInstruction.into());
        }
        permission_info.is_initialized = true;
        permission_info.roles[0].key = *signer.key;
        permission_info.roles[0].role = ROLE_OWNER;
        msg!("set things");

        PermissionState::pack(&permission_info, &mut permission_account.data.borrow_mut());

        Ok(())
    }

    fn process_set_value(
        accounts: &[AccountInfo],
        role: u8,
        _program_id: &Pubkey,
    ) -> ProgramResult {
        msg!("Started set value");
        let account_info_iter = &mut accounts.iter();

        let signer = next_account_info(account_info_iter)?;
        //verify signer
        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let permission_account = next_account_info(account_info_iter)?;
        let _rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        let update_account = next_account_info(account_info_iter)?;
        msg!("got accoutns");

        // State must already be initialized
        let mut permission_info = PermissionState::unpack(&permission_account.data.borrow())?;
        if !permission_info.is_initialized() {
            return Err(PermissionError::InvalidInstruction.into());
        }
        msg!("unpacked");

        //find index of signer role/ account we are looking for & empty slot
        let mut signer_index: isize = -1;
        let mut account_index: isize = -1;
        let mut empty_index: isize = -1;
        for i in 0..8 {
            if permission_info.roles[i].key == *signer.key{
                signer_index = i as isize;
            }
            if permission_info.roles[i].key == *update_account.key{
                account_index = i as isize;
            }
            if permission_info.roles[i].key == Pubkey::default(){
                empty_index = i as isize;
            }
        }

        //check signer perm
        if signer_index < 0 {
            msg!("Invalid permission");
            return Err(PermissionError::InvalidPermission.into());
        }
        let existing_role = permission_info.roles[signer_index as usize].role;
        msg!("Got role {}", existing_role);
        if !does_role_have_permission(existing_role, PERMISSION_DELEGATEADD){
            msg!("Invalid permission2");
            return Err(PermissionError::InvalidPermission.into());
        }

        //update permissions for other accounts
        if account_index < 0 {
            if empty_index < 0 {
                msg!("Permission array full");
                return Err(PermissionError::PermissionFull.into())
            }
            account_index = empty_index;
            permission_info.roles[account_index as usize].key = *update_account.key;
        }
        //update permission
        permission_info.roles[account_index as usize].role = role;
        
        PermissionState::pack(&permission_info, &mut permission_account.data.borrow_mut());
        
        Ok(())
    }
}
