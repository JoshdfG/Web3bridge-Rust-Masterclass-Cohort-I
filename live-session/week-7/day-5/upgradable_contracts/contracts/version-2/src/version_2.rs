use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
enum Datakey {
    Owner,
    NewOwner,
}

#[contract]
pub struct UpgradableContractV2;

#[contractimpl]
impl UpgradableContractV2 {
    pub fn __constructor(env: Env, owner: Address) {
        env.storage().persistent().set(&Datakey::Owner, &owner);
        env.storage().persistent().set(&Datakey::NewOwner, &owner);
    }

    pub fn init(env: Env) {
        let owner: Address = env.storage().persistent().get(&Datakey::Owner).unwrap();
        owner.require_auth();

        if !env.storage().persistent().has(&Datakey::NewOwner) {
            env.storage().persistent().set(&Datakey::NewOwner, &owner);
        }
    }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

    /// Upgrade syntax/method in soroban
    /// V1 -- swaps to -> V2 but v2 still need to have
    /// the upgrade function so incase there is an upgrade to be made
    /// you can easily upgrade that also.
    pub fn upgrade_v2(env: Env, new_wasm_hash: BytesN<32>) {
        let owner: Address = env.storage().persistent().get(&Datakey::NewOwner).unwrap();
        owner.require_auth();

        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
