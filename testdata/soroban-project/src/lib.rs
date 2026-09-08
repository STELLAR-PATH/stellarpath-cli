#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello() {}
}
