use anchor_lang::prelude::{
  error_code,
};

#[error_code]
pub enum ErrorCode {

  #[msg("RequestMultiplexer: Invalid input.")]
  InvalidInput,

  #[msg("RequestMultiplexer: Unauthorized.")]
  Unauthorized,
}
