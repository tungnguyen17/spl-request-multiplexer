use anchor_lang::prelude::{
  Account,
  AccountInfo,
  Accounts,
  AnchorDeserialize,
  borsh,
  Key,
  Program,
  Pubkey,
  Rent,
  SolanaSysvar,
  System,
  ToAccountInfo,
};
use crate::state::{
  Group,
};

#[derive(Accounts)]
#[instruction(derivation_path: Vec<u8>, member_count: u16)]
pub struct CreateGroupContext<'info> {

  /// CHECK: Any account
  #[account(signer, mut)]
  pub owner: AccountInfo<'info>,

  #[account(
    init,
    seeds = [
      &[52, 202, 14, 118, 96, 136, 66, 255],
      &*derivation_path,
    ],
    bump,
    space = 16 + Group::size(member_count),
    payer = owner,
  )]
  pub group: Account<'info, Group>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGroupContext<'info> {

  /// CHECK: Group owner, verify using #access_control
  #[account(signer)]
  pub owner: AccountInfo<'info>,

  #[account(mut)]
  pub group: Account<'info, Group>,
}
