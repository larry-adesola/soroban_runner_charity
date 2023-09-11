#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short,Address, Vec, Env, Symbol};

mod donor {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/donor.wasm"
    );
}

const DISTANCE: Symbol = symbol_short!("DISTANCE");
const TOTALCASH: Symbol = symbol_short!("TOTALCASH");
const DONORS: Symbol = symbol_short!("DONORS");

#[contract]
pub struct RunnerContract;

#[contractimpl]
impl RunnerContract {
    /// Increment an internal counter; return the new value.
    pub fn increment_distance(env: Env) -> u32 {
        let mut distance: u32 = env.storage().instance().get(&DISTANCE).unwrap_or(0);
        let mut total_cash: u32 = env.storage().instance().get(&TOTALCASH).unwrap_or(0);

        distance += 1;

        log!(&env, "count: {}", distance);

        env.storage().instance().set(&DISTANCE, &distance);

        

        let mut donors: Vec<Address> = env
        .storage()
        .instance()
        .get(&DONORS)
        .unwrap_or(Vec::new(&env));

        for don in donors.iter() {
            match don {
                _ => {
                    let client = donor::Client::new(&env, &don);
                    let to_add = 0;//client.tester(); //client.pay(&distance);
                    if to_add > 0{
                        distance == 10;
                    }
                    distance == 100;
                    total_cash += to_add;
                }
                //_ => ()
            }

        }
        env.storage().instance().set(&DISTANCE, &distance);
        env.storage().instance().set(&TOTALCASH, &total_cash);
        env.storage().instance().bump(1000);
        distance
    }
    pub fn money_made(env: Env) -> u32{
        let mut total_cash: u32 = env.storage().instance().get(&TOTALCASH).unwrap_or(0);
        env.storage().instance().bump(1000);
        total_cash
    }
    pub fn distance(env: Env) -> u32{
        let mut distance: u32 = env.storage().instance().get(&DISTANCE).unwrap_or(0);
        env.storage().instance().bump(1000);
        distance
    }

    pub fn end_run(env: Env) -> u32{
        let mut distance: u32 = env.storage().instance().get(&DISTANCE).unwrap_or(0);
        let mut total_cash: u32 = env.storage().instance().get(&TOTALCASH).unwrap_or(0);
        env.storage().instance().bump(1000);
        total_cash
    }
    pub fn get_donors(env: Env) ->  Vec<Address>{
        let donors: Vec<Address>= env
        .storage()
        .instance()
        .get(&DONORS)
        .unwrap_or(Vec::new(&env))
    ;
    donors
    }
    pub fn add_donor(env: Env, addr: Address)-> u32 {
        let mut donors: Vec<Address> = env
        .storage()
        .instance()
        .get(&DONORS)
        .unwrap_or(Vec::new(&env));

        if ! donors.contains(&addr) {
            donors.push_back(addr);
            env.storage().instance().set(&DONORS, &donors);
        }

        donors.len() as u32

    }
    



}

#[cfg(test)]
mod test;