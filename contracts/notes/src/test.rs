#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{SariSendContract, SariSendContractClient};

#[test]
fn test_happy_path_payment() {
    let env = Env::default();

    let contract_id = env.register_contract(None, SariSendContract);
    let client = SariSendContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.create_debt(&user, &1000);
    client.pay_debt(&user, &1000);

    let debt = client.get_debt(&user);

    assert_eq!(debt.amount_due, 0);
    assert_eq!(debt.paid, true);
}

#[test]
#[should_panic]
fn test_nonexistent_debt() {
    let env = Env::default();

    let contract_id = env.register_contract(None, SariSendContract);
    let client = SariSendContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.pay_debt(&user, &500);
}

#[test]
fn test_storage_updated() {
    let env = Env::default();

    let contract_id = env.register_contract(None, SariSendContract);
    let client = SariSendContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.create_debt(&user, &2000);
    client.pay_debt(&user, &500);

    let debt = client.get_debt(&user);

    assert_eq!(debt.amount_due, 1500);
}

#[test]
fn test_partial_payment() {
    let env = Env::default();

    let contract_id = env.register_contract(None, SariSendContract);
    let client = SariSendContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.create_debt(&user, &3000);
    client.pay_debt(&user, &1000);

    let debt = client.get_debt(&user);

    assert_eq!(debt.paid, false);
}

#[test]
fn test_paid_status() {
    let env = Env::default();

    let contract_id = env.register_contract(None, SariSendContract);
    let client = SariSendContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.create_debt(&user, &500);
    client.pay_debt(&user, &500);

    let status = client.is_paid(&user);

    assert_eq!(status, true);
}