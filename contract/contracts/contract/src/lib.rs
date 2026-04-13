#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Address};

#[contract]
pub struct RoyaltyEngine;

#[contractimpl]
impl RoyaltyEngine {

    // Store royalty percentage for a creator
    pub fn set_royalty(env: Env, creator: Address, percentage: u32) {
        let key = (symbol_short!("ROYALTY"), creator.clone());
        env.storage().instance().set(&key, &percentage);
    }

    // Get royalty percentage of a creator
    pub fn get_royalty(env: Env, creator: Address) -> u32 {
        let key = (symbol_short!("ROYALTY"), creator);
        env.storage().instance().get(&key).unwrap_or(0)
    }

    // Calculate royalty amount from a payment
    pub fn calculate_royalty(env: Env, creator: Address, amount: i128) -> i128 {
        let percentage = Self::get_royalty(env, creator);
        (amount * percentage as i128) / 100
    }
}