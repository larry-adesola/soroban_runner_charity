use crate::{RunnerContract, RunnerContractClient, donor};
use soroban_sdk::Env;
use soroban_sdk::log;
use soroban_sdk::Address;
use soroban_sdk::Vec;
extern crate std;

#[test]
fn runner() {
    let env = Env::default();
    let contract_id = env.register_contract(None, RunnerContract);
    let client = RunnerContractClient::new(&env, &contract_id);
    // Register contract A using the imported Wasm.
    let donor_id_1 = env.register_contract_wasm(None, donor::WASM);
    let donor_id_2 = env.register_contract_wasm(None, donor::WASM);

    let client_1 = donor::Client::new(&env, &donor_id_1);
    client_1.donate(&contract_id, &100, &2);
    
    client.increment_distance();
    client.increment_distance();
    client.increment_distance();
    let sum = client.add_donor(&donor_id_1);
    client.increment_distance();
    

    //let client_2 = donor::Client::new(&env, &donor_id_2);

    // Register contract B defined in this crate.

    // Create a cl
    
    //let sum = client.add_donor(&donor_id_2);
    //let sum = client.get_donors();
    let sum = client.money_made();
    //let sum = client.distance();
    //assert_eq!(sum, Vec::new(&env));
    assert_eq!(1,sum);
}