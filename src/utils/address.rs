#![allow(missing_docs)]

use soroban_sdk::{Address, Env, String};

use crate::types::ContractError;

/// The Stellar "zero" account address (G... strkey for an all-zero public key).
/// Passing this address to administrative functions must be rejected to avoid
/// black-holing permissions or funds.
pub const ZERO_ADDRESS_STR: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";

/// Reject the Stellar zero-address.
///
/// Returns [`ContractError::InvalidAddress`] when `address` matches the
/// all-zero account strkey. This is the centralized sanitizer referenced in
/// issue #126.
pub fn validate_address(env: &Env, address: &Address) -> Result<(), ContractError> {
    let zero = String::from_str(env, ZERO_ADDRESS_STR);
    if address.to_string() == zero {
        return Err(ContractError::InvalidAddress);
    }
    Ok(())
}
