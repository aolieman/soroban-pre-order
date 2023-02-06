#![no_std]
/// For SDK documentation see: https://soroban.stellar.org/docs/sdks/rust-auth
use soroban_auth::{Identifier, Signature};
/// For SDK documentation see: https://soroban.stellar.org/docs/sdks/rust-sdk
use soroban_sdk::{contracterror, contractimpl, contracttype, AccountId, Env};

/// use the contractimport macro to generate a module with the built-in token contract
mod token {
    soroban_sdk::contractimport!(file = "./soroban_token_spec.wasm");
}


/// the `trait` specifies the interface that the `PreorderContract` must adhere to
pub trait PreorderTrait {
    // set up the terms of a specific sale that accepts pre-orders
    fn initialize(
        e: Env,
        recipient: AccountId, // the account to which the payment of fufilled orders will be sent
        token_id: BytesN<32>, // the id of the token being that is accepted as payment
        lock_period: u64,     // the amount of time that needs to pass before a refund becomes available
    ) -> Result<(), Error>;

    // refund payment of an expired order to myself
    fn refund(e: Env) -> Result<(), Error>;
}

pub struct PreorderContract;