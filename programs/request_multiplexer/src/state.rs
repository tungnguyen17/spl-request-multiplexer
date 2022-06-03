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

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InstructionAccount {
  pub pubkey: Pubkey,
  pub is_signer: bool,
  pub is_writable: bool,
}

impl From<&InstructionAccount> for AccountMeta {
  fn from(account: &InstructionAccount)
  -> AccountMeta {
    match account.is_writable {
      false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
      true => AccountMeta::new(account.pubkey, account.is_signer),
    }
  }
}
