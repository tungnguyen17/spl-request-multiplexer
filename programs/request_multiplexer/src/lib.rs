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
}

pub fn is_owner(user: &Pubkey, group: &Group) -> Result<()> {
  if *user != group.owner {
    return Err(ErrorCode::Unauthorized.into());
  }

  Ok(())
}
