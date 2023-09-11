#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, Address};

mod runner {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/runner.wasm"
    );
}

const AMOUNT : Symbol = symbol_short!("AMOUNT");
const QUOTA : Symbol = symbol_short!("QUOTA");

#[contract]
pub struct DonorContract;

#[contractimpl]
impl DonorContract {
    /// Say Hello to someone or something.
    /// Returns a length-2 vector/array containing 'Hello' and then the value passed as `to`.
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
    pub fn add( x: u32, y:u32) -> u32{
        x.checked_add(y).expect("no overflow")
    }
    pub fn donate(env : Env, escrow : u32, distance : u32) -> u32{
        let mut amount : u32 = env.storage().instance().get(&AMOUNT).unwrap_or(0);
        let mut quota: u32 = env.storage().instance().get(&QUOTA).unwrap_or(0);

        quota  = distance;
        amount = escrow;

        env.storage().instance().set(&AMOUNT, &amount);
        env.storage().instance().set(&QUOTA, &QUOTA);

        env.storage().instance().bump(1000);
        //let client = runner::Client::new(&env, &runner);
        //client.add_donor(&self);
        amount
    }
    pub fn tester(env : Env) -> u32{
        1 as u32
    }

    pub fn pay(env : Env, distanceCovered : u32) -> u32{
        let amount : u32 = env.storage().instance().get(&AMOUNT).unwrap_or(0);
        let mut quota: u32 = env.storage().instance().get(&QUOTA).unwrap_or(0);

        if distanceCovered >= quota {
            let temp : u32 = 0;
            env.storage().instance().set(&AMOUNT, &temp);
            amount
        }else{
            0 as u32
        }
    }
}
#[cfg(test)]
mod test;