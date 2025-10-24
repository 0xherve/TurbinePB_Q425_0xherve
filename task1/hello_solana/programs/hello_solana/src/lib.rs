//Program that gives a hello solana message

use anchor_lang::prelude::*;

declare_id!("Gfpdhj6NVVnbrUa8bUb8gYXsU999Pj7iuZUdzYvXkSvs");

#[program]
pub mod hello_solana {
    use super::*;

pub fn hellosolana(_ctx: Context<HelloSolana>) -> Result<()> {
    msg!("Hello, Solana!");
    Ok(())
}
}

#[derive(Accounts)]
pub struct HelloSolana {}
