use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{Pack, Sealed},
};

#[repr(C)]
pub struct MathResult {
    pub sum: u64,
    pub difference: u64,
}

impl Sealed for MathResult {}
impl Pack for MathResult {
    const LEN: usize = 16; 

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let sum = u64::from_le_bytes(src[0..8].try_into().unwrap());
        let difference = u64::from_le_bytes(src[8..16].try_into().unwrap());

        Ok(MathResult { sum, difference })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0..8].copy_from_slice(&self.sum.to_le_bytes());
        dst[8..16].copy_from_slice(&self.difference.to_le_bytes());
    }
}

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Math program entrypoint");

    let operation = instruction_data[0];

    let accounts_iter = &mut accounts.iter();
    let result_account = next_account_info(accounts_iter)?;

    if result_account.owner != program_id {
        msg!("Result account not owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if result_account.data_len() != MathResult::LEN {
        msg!("Result account data size incorrect");
        return Err(ProgramError::InvalidAccountData);
    }

    let mut result_data = MathResult::unpack_from_slice(&result_account.data.borrow())?;

    let num1 = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
    let num2 = u64::from_le_bytes(instruction_data[9..17].try_into().unwrap());

    match operation {
        1 => {

            result_data.sum = num1 + num2;
        }
        2 => {
            result_data.difference = num1 - num2;
        }
        _ => {
            msg!("Invalid operation");
            return Err(ProgramError::InvalidInstructionData);
        }
    }
    result_data.pack_into_slice(&mut result_account.data.borrow_mut());

    Ok(())
}
