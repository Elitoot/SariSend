#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone)]
pub struct DebtRecord {
    pub debtor: Address,
    pub amount_due: i128,
    pub paid: bool,
}

#[contracttype]
pub enum DataKey {
    Debt(Address),
}

#[contract]
pub struct SariSendContract;

#[contractimpl]
impl SariSendContract {

    // Create a new debt entry for a customer
    pub fn create_debt(
        env: Env,
        debtor: Address,
        amount: i128,
    ) {
        debtor.require_auth();

        let debt = DebtRecord {
            debtor: debtor.clone(),
            amount_due: amount,
            paid: false,
        };

        env.storage().persistent().set(
            &DataKey::Debt(debtor),
            &debt,
        );
    }

    // Pay part or full debt
    pub fn pay_debt(
        env: Env,
        debtor: Address,
        payment: i128,
    ) {
        debtor.require_auth();

        let key = DataKey::Debt(debtor.clone());

        let mut debt: DebtRecord = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if payment >= debt.amount_due {
            debt.amount_due = 0;
            debt.paid = true;
        } else {
            debt.amount_due -= payment;
        }

        env.storage().persistent().set(&key, &debt);
    }

    // View debt information
    pub fn get_debt(
        env: Env,
        debtor: Address,
    ) -> DebtRecord {
        env.storage()
            .persistent()
            .get(&DataKey::Debt(debtor))
            .unwrap()
    }

    // Check if debt fully paid
    pub fn is_paid(
        env: Env,
        debtor: Address,
    ) -> bool {
        let debt: DebtRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Debt(debtor))
            .unwrap();

        debt.paid
    }
}