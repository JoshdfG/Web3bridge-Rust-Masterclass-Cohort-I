use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
enum Datakey {
    Owner,
}

#[contract]
pub struct UpgradableContractV1;

#[contractimpl]
impl UpgradableContractV1 {
    pub fn __constructor(env: Env, owner: Address) {
        env.storage().persistent().set(&Datakey::Owner, &owner);
    }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn upgrade_v1(env: Env, new_wasm_hash: BytesN<32>) {
        let owner: Address = env.storage().persistent().get(&Datakey::Owner).unwrap();
        owner.require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
