use super::*;
use std::collections::HashMap;
use bs58::{decode, encode};
use crate::programs::sys;

fn run() -> ProgramResult {
    start(sys::ID)?;
    start(token::ID)?;
    start(mint::ID)?;

    Ok(())
}

pub fn start(program_id: Pubkey) -> ProgramResult {
    // match program_id {
    //     sys::ID => sys::entrypoint(&sys::ID, &mut sys::get_context()?),
    //     token::ID => token::entrypoint(&token::ID, &mut token::get_context()?),
    //     mint::ID => mint::entrypoint(&mint::ID, &mut mint::get_context()?),
    //     _ => Err(entrypoint::ProgramError::InvalidAccountData),
    // }
    Ok(())
}