use anchor_lang::prelude::*;

#[account]
pub struct Group {
  pub owner: Pubkey,
  pub new_owner: Pubkey,
  pub members: Vec<Pubkey>,
}

impl Group {
  pub fn size(member_count: u16) -> usize {
    32 + 32 + (4 + 32 * usize::from(member_count))
  }
}
