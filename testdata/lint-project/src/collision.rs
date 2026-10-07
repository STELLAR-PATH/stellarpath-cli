#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

#[contract]
pub struct CollisionContract;

#[contractimpl]
impl CollisionContract {
    pub fn save(env: Env) {
        env.storage().persistent().set(&symbol_short!("key"), &1);
    }
}
