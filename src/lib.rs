use std::fmt::Error;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, instruction::AccountMeta, msg, program_error::ProgramError, pubkey::Pubkey,entrypoint
};

// Instruction enum: defines what actions are supported
#[derive(BorshDeserialize, BorshSerialize)]
pub enum InstructionType {
    Increment(u32),
    Decrement(u32),
    Multiply(u32),
    Divide(u32)
}

// Account data structure
#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

// Solana program entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],      // Array of accounts the instruction will interact with
    instruction_data: &[u8],       // Serialized instruction payload
) -> ProgramResult {
    // let acc =  match next_account_info(&mut accounts.iter()){
    //     Ok(acc) => acc,
    //     Err(e) => return Err(e),
    // };//what does question mark means -> if the inner data exist then result or return the outer ProgramResult error

    let acc = next_account_info(&mut accounts.iter())?;

    // Deserialize instruction payload
    // let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let instruction_type = match InstructionType::try_from_slice(instruction_data) {
    Ok(instruction_type) => instruction_type,
    Err(err) => return Err(err.into()),
};

    
    // Deserialize counter account data
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?; // Borrow kar rahe Focus here 

    // Apply logic based on instruction type
    match instruction_type {
        InstructionType::Increment(value) => {
            counter_data.count = counter_data.count
                .checked_add(value)
                .ok_or(ProgramError::InvalidInstructionData)?;
        }
        InstructionType::Decrement(value) => {
            counter_data.count = counter_data.count
                .checked_sub(value)
                .ok_or(ProgramError::InvalidInstructionData)?;
        }
        InstructionType::Multiply(value) => {
            counter_data.count = counter_data.count
                 .checked_mul(value)
                 .ok_or(ProgramError::InvalidInstructionData)?;
        }
        InstructionType::Divide(value) => {
            if value == 0 {
                msg!("Division by zero");
                return Err(ProgramError::InvalidInstructionData)?;
            }
            counter_data.count = counter_data.count
                  .checked_div(value)
                  .ok_or(ProgramError::InvalidInstructionData)?;
            }
        }
    

    // Serialize the updated data back to the account
    counter_data.serialize(&mut *acc.data.borrow_mut())?;

    msg!("contract succeeded");

    Ok(())
}





// use borsh::{BorshDeserialize, BorshSerialize, to_vec};

// #[derive(Debug, BorshDeserialize, BorshSerialize)]
// struct User {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let u = User {
//         name: String::from("Raunit"),
//         age: 21,
//     };

//     // Serialize the struct into bytes
//     let bytes = to_vec(&u).unwrap();

//     // Deserialize the bytes back into a struct
//     let u2 = User::try_from_slice(&bytes).unwrap();

//     println!("{:?}", u);
//     println!("{:?}", u2);
// }
