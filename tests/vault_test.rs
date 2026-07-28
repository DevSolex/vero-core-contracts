use soroban_sdk::{
    testutils::{Address as _},
    Address, Env,
};
use vero_core_contracts::VeroContractClient;

const LOCK_THRESHOLD: i128 = 100;

fn setup() -> (Env, Address, Address, VeroContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, vero_core_contracts::VeroContract);
    let client = VeroContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin);
    let token_addr = token.address();

    client.initialize(&admin, &token_addr, &LOCK_THRESHOLD);

    (env, admin, token_addr, client)
}

fn add_guardian_with_rep(
    env: &Env,
    client: &VeroContractClient,
    admin: &Address,
    score: u64,
) -> Address {
    let guardian = Address::generate(env);
    client.add_guardian(admin, &guardian);
    client.set_reputation(admin, &guardian, &score);
    guardian
}

#[test]
fn test_task_resolves_when_vault_fails() {
    let (env, admin, _token_addr, client) = setup();

    // Get the contract ID for the vault
    let contract_id = env.register_contract(None, vero_core_contracts::VeroContract);
    
    // Set the vault to the contract itself (which doesn't implement release_funds properly)
    // This will cause the vault call to fail
    client.set_vault_address(&admin, &contract_id);

    // Add a guardian with reputation
    let guardian = add_guardian_with_rep(&env, &client, &admin, 500);

    // Set weight threshold to 300
    client.set_weight_threshold(&admin, &300u64);

    // Register a task
    client.register_task(&admin, &1u64, &1u32);

    // Cast a vote that should resolve the task
    // The vault call will fail, but with our fix, the task should still resolve
    client.vote(&guardian, &1u64);

    // Verify the task is done even though the vault call failed
    let task = client.get_task(&1u64).unwrap();
    assert!(task.is_done, "Task should be resolved even when vault call fails");
    assert_eq!(task.votes, 1);
    assert_eq!(task.total_weight_accrued, 500);
}
