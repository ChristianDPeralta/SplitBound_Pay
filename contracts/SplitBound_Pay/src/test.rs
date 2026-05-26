#![cfg(test)]

mod tests {
    use super::super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_1_happy_path_successful_settlement() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, SplitBoundContract);
        let client = SplitBoundContractClient::new(&env, &contract_id);

        let manager_maya = Address::generate(&env);
        let roommate_1 = Address::generate(&env);
        let roommate_2 = Address::generate(&env);
        let utility_company = Address::generate(&env);

        // Setup utility bill of 150 tokens with deadline of 50 blocks
        client.setup_bill(&manager_maya, &150, &50);

        // Roommates contribute their splits
        client.deposit_share(&roommate_1, &75);
        client.deposit_share(&roommate_2, &75);

        // Complete funds hit target; contract triggers transfer to utility firm
        let settlement_payout = client.settle_utility(&utility_company);
        assert_eq!(settlement_payout, 150);
    }

    #[test]
    #[should_panic(expected = "Total collected falls short of bill requirement")]
    fn test_2_edge_case_insufficient_pool_funds() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, SplitBoundContract);
        let client = SplitBoundContractClient::new(&env, &contract_id);

        let manager_maya = Address::generate(&env);
        let roommate_1 = Address::generate(&env);
        let utility_company = Address::generate(&env);

        client.setup_bill(&manager_maya, &200, &50);
        client.deposit_share(&roommate_1, &80);

        // Payout must crash because 80 collected tokens do not fulfill the 200 required target
        client.settle_utility(&utility_company);
    }

    #[test]
    fn test_3_state_verification() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, SplitBoundContract);
        let client = SplitBoundContractClient::new(&env, &contract_id);

        let manager_maya = Address::generate(&env);
        let roommate_1 = Address::generate(&env);

        client.setup_bill(&manager_maya, &300, &100);
        client.deposit_share(&roommate_1, &100);

        // Explicitly assert that internal storage accurately reflects the on-chain operations
        let checked_total = env.as_contract(&contract_id, || {
            env.storage().instance().get(&StorageKey::AmountPooled).unwrap_or(0i128)
        });
        assert_eq!(checked_total, 100);
    }

    #[test]
    #[should_panic(expected = "The deadline for this billing cycle has passed")]
    fn test_4_deposit_attempt_past_deadline() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, SplitBoundContract);
        let client = SplitBoundContractClient::new(&env, &contract_id);

        let manager_maya = Address::generate(&env);
        let roommate_1 = Address::generate(&env);

        client.setup_bill(&manager_maya, &150, &20);

        // Advance environmental block properties to simulate missing the deadline window
        env.ledger().with_mut(|info| {
            info.sequence = 25;
        });

        client.deposit_share(&roommate_1, &75);
    }

    #[test]
    #[should_panic(expected = "Bill total must be positive")]
    fn test_5_invalid_negative_billing_setup() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, SplitBoundContract);
        let client = SplitBoundContractClient::new(&env, &contract_id);

        let manager_maya = Address::generate(&env);
        
        // Setting up a negative cost profile should fail contract assertions immediately
        client.setup_bill(&manager_maya, &-100, &40);
    }
}