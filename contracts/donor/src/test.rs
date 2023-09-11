//use crate::{Contract, ContractClient};
use crate::{DonorContract, DonorContractClient, runner};
use soroban_sdk::Env;
use soroban_sdk::log;
use soroban_sdk::Address;
use soroban_sdk::Vec;
extern crate std;
use soroban_sdk::symbol_short;

#[test]
fn hello() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DonorContract);
    let client = DonorContractClient::new(&env, &contract_id);

    assert_eq!(
        1,1
    );
}