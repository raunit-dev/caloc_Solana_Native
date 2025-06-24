use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    //next_account_info -> Function for fetching the next account from the input accounts
    //AccountInfo -> Struct which represents what an input account looks like
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,//return type of the main function
    msg,//same as println
    pubkey::Pubkey,//pubkey struct represents a public key
    entrypoint//macro use to specify the entrypoint fn
};


entrypoint!(process_instructions);

pub fn process_instruction(
   _program_id: &Pubkey,
   accounts: &[AccountInfo],//An array of input addresses.This array should have all the addresses the user is going to interact
   //with when they are calling this function
   instruction_data: &[u8],//the set of actual thing user wants to do,its an array of bytes but it can be deserialized into a struct This contains information like what to do 
) -> ProgramResult {

}