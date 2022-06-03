pub mod context;
pub mod error;
pub mod state;

use anchor_lang::prelude::*;
use crate::context::*;
use crate::error::{
  ErrorCode,
};
use crate::state::{
  Group,
  InstructionAccount,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod request_multiplexer {
  use super::*;

  pub fn create_group(
    ctx: Context<CreateGroupContext>,
    _derivation_path: Vec<u8>,
    member_count: u16,
  ) -> Result<()> {

    if member_count < 5 {
      return Err(ErrorCode::InvalidInput.into());
    }

    let owner = &ctx.accounts.owner;

    let group = &mut ctx.accounts.group;

    group.owner = owner.key();
    group.members = Vec::new();

    Ok(())
  }

  #[access_control(is_owner(&ctx.accounts.owner.key, &ctx.accounts.group))]
  pub fn update_group(
    ctx: Context<UpdateGroupContext>,
    members: Vec<Pubkey>,
  ) -> Result<()> {

    let group = &mut ctx.accounts.group;

    group.members = members;

    Ok(())
  }

  #[access_control(is_member(&ctx.accounts.member.key, &ctx.accounts.group))]
  pub fn execute_instruction(
    ctx: Context<ExecuteInstructionContext>,
    target_program_id: Pubkey,
    instruction_accounts: Vec<InstructionAccount>,
    instruction_data: Vec<u8>,
  ) -> Result<()> {

    let group = &ctx.accounts.group;

    let (signer, signer_nonce) = Pubkey::find_program_address(
      &[
        &[2, 151, 229, 53, 244, 77, 229, 7],
        &group.key().as_ref(),
      ],
      &ctx.program_id,
    );

    let instruction_accounts: Vec<AccountMeta> = instruction_accounts.clone()
      .iter()
      .map(|acc| {
        if acc.pubkey == signer {
          if acc.is_writable {
            AccountMeta::new(acc.pubkey, true)
          }
          else {
            AccountMeta::new_readonly(acc.pubkey, true)
          }
        } else {
          acc.into()
        }
      })
      .collect();

    let instruction = solana_program::instruction::Instruction {
      data: instruction_data.clone(),
      accounts: instruction_accounts,
      program_id: target_program_id,
    };

    let accounts = ctx.remaining_accounts;

    let seeds: &[&[u8]] = &[
      &[2, 151, 229, 53, 244, 77, 229, 7],
      group.to_account_info().key.as_ref(),
      &[signer_nonce],
    ];

    solana_program::program::invoke_signed(&instruction, &accounts, &[&seeds])
      .expect("RequestMultiplexer: CPI failed.");

    Ok(())
  }
}

pub fn is_owner(user: &Pubkey, group: &Group) -> Result<()> {
  if *user != group.owner {
    return Err(ErrorCode::Unauthorized.into());
  }

  Ok(())
}

pub fn is_member(user: &Pubkey, group: &Group) -> Result<()> {
  if *user == group.owner {
   return Ok(());
  }

  let result = group.members.iter().position(|&key| key == *user);
  if result == None {
    return Err(ErrorCode::Unauthorized.into());
  }

  Ok(())
}
