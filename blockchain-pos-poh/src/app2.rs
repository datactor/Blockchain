use super::*;
use std::collections::HashMap;
use bs58::{decode, encode};
use crate::programs::sys;

fn run() -> ProgramResult {
    start(sys::SYS_ID)?;
    start(token::ID)?;
    start(mint::ID)?;

    Ok(())
}

pub fn start(program_id: Pubkey) -> ProgramResult {
    match program_id {
        // sys::ID => {},
        // token::ID => {},
        // mint::ID => {},
        _ => Err(entrypoint::ProgramError::InvalidAccountData),
    }?;
    Ok(())
}