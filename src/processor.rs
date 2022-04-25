use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use spl_token::state::Account as TokenAccount;

pub struct Processor;

impl Processor{
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        let instruction = StakingInstruction::unpack(instruction_data)?;

        match instruction{
            StakingInstruction::InitCollection{} => {
                msg!("Instruction: InitCollection");
                Self::process_init_collection(accounts,authenticate,rate,balance,program_id)
            }

            StakingInstruction::Lock {} => {
                msg!("Instruction: Lock");
                Self::process_locknft(duration)
            }
        }


    }


fn process_init_collection(
    accounts: &[AccountInfo],
    authenticate: bool, //Verify creator address
    rate: u8,
    balance: u64,
    program_id: &Pubkey,

) -> ProgramResult{
    Ok(())

}

fn process_locknft(
    duration: Time,


) -> ProgramResult{
    Ok(())
}

fn process_withdraw(
    verify: Pubkey, //
    duration: Time,

) -> ProgramResult{
    Ok(())
}

fn process_unstake(

) -> ProgramResult{
    Ok(())
}

fn process_whitelist(


) -> ProgramResult{
    Ok(())
}

}

