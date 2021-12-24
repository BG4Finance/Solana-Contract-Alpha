/// Instruction.rs
use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {

    /// Start the trade by creating an populating an escrow account. Then transfer the ownership of the given temp token account to the PDA
    /// 
    /// 
    /// Accounts expected: 
    /// 
    /// 0. `[signer]` account of who is initialising the the escrow
    /// 1. `[writable]` temporary token account. Created prior this instruction and owned by the initializer. Ownership will be transfered to the escrow
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account. It will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token prograram
    InitEscrow{
        /// Amount party A expects to receive of token Y
        amount: u64
    }
}

impl EscrowInstruction {
    /// Unpack
    pub fn unpack(input: & [u8])->Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}

