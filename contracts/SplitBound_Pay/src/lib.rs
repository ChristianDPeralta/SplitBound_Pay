#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, log};

// Unique keys utilized inside Soroban's instance storage to manage state variables securely.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    Manager,             // Account authorized to initialize and manage the utility bill parameters.
    BillTotal,           // Total amount of tokens required to satisfy and settle the billing requirement.
    BillingDeadline,     // Target ledger block sequence number when deposits are frozen.
    AmountPooled,        // Rolling tally of total roommate asset tokens collected so far.
    TenantPaid(Address), // Mapping tracking exactly how much each roommate has individually sent.
    SettledStatus,       // Status flag specifying if the final utility payout has occurred.
}

#[contract]
pub struct SplitBoundContract;

#[contractimpl]
impl SplitBoundContract {
    /// Initializes the shared household billing ledger with the total cost and block deadline.
    /// This establishes the baseline target that roommates must collectively fund.
    pub fn setup_bill(env: Env, manager: Address, total_bill: i128, due_block: u32) {
        // Enforce initialization guard rail: prevent overwriting an active billing instance.
        assert!(!env.storage().instance().has(&StorageKey::Manager), "Bill ledger already configured");
        assert!(total_bill > 0, "Bill total must be positive");

        env.storage().instance().set(&StorageKey::Manager, &manager);
        env.storage().instance().set(&StorageKey::BillTotal, &total_bill);
        env.storage().instance().set(&StorageKey::BillingDeadline, &due_block);
        env.storage().instance().set(&StorageKey::AmountPooled, &0i128);
        env.storage().instance().set(&StorageKey::SettledStatus, &false);
        
        log!(&env, "Household bill ledger successfully configured by manager");
    }

    /// Roommates call this function to deposit their designated financial split directly into escrow.
    /// It updates individual tracking and aggregates the total pool balance toward settlement.
    pub fn deposit_share(env: Env, tenant: Address, amount: i128) {
        tenant.require_auth();
        assert!(amount > 0, "Deposit amount must be positive");
        
        // Security check: Verify that the utility payment has not already been finalized.
        let is_settled: bool = env.storage().instance().get(&StorageKey::SettledStatus).unwrap_or(false);
        assert!(!is_settled, "This monthly ledger has already been settled");

        // Time constraint validation: Verify the ledger block sequence has not passed the deadline.
        let current_block = env.ledger().sequence();
        let deadline: u32 = env.storage().instance().get(&StorageKey::BillingDeadline).unwrap_or(0);
        assert!(current_block <= deadline, "The deadline for this billing cycle has passed");

        // Record individual roommate payment progress.
        let current_tenant_paid: i128 = env.storage().instance().get(&StorageKey::TenantPaid(tenant.clone())).unwrap_or(0);
        let new_tenant_paid = current_tenant_paid + amount;
        env.storage().instance().set(&StorageKey::TenantPaid(tenant.clone()), &new_tenant_paid);

        // Update overall escrow pool collection ledger balance.
        let overall_pooled: i128 = env.storage().instance().get(&StorageKey::AmountPooled).unwrap_or(0);
        let new_overall = overall_pooled + amount;
        env.storage().instance().set(&StorageKey::AmountPooled, &new_overall);

        log!(&env, "Roommate share tracked successfully on-chain");
    }

    /// Releases the pooled escrow contract funds directly to the registered utility provider.
    /// Requires manager signature and that the roommates have pooled 100% of the target bill.
    pub fn settle_utility(env: Env, provider: Address) -> i128 {
        // Enforce manager identity checks before releasing capital assets.
        let manager: Address = env.storage().instance().get(&StorageKey::Manager).expect("Ledger not setup");
        manager.require_auth();

        let amount_pooled: i128 = env.storage().instance().get(&StorageKey::AmountPooled).unwrap_or(0);
        let bill_total: i128 = env.storage().instance().get(&StorageKey::BillTotal).unwrap_or(0);
        
        // Safety lock: Contract blocks settlement if pooled amounts fall short of the total bill.
        assert!(amount_pooled >= bill_total, "Total collected falls short of bill requirement");
        
        let is_settled: bool = env.storage().instance().get(&StorageKey::SettledStatus).unwrap_or(false);
        assert!(!is_settled, "Bill already settled");

        env.storage().instance().set(&StorageKey::SettledStatus, &true);
        
        log!(&env, "Utility payment validated. Releasing entire pooled balance to provider");
        amount_pooled
    }
}