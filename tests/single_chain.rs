// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Integration testing for the depin_demo application.

#![cfg(not(target_arch = "wasm32"))]

use depin_demo::Operation;
use linera_sdk::test::TestValidator;

/// Tests setting and incrementing a counter
///
/// Creates the application on a `chain`, initializing it with a 10 then add 10 and obtain 20.
/// which is then checked.
#[tokio::test(flavor = "multi_thread")]
async fn single_chain_test() {
    let (validator, bytecode_id) =
        TestValidator::with_current_bytecode::<depin_demo::DepinDemoAbi, (), u64>().await;
    let mut chain = validator.new_chain().await;

    let initial_state = 10u64;
    let application_id = chain
        .create_application(bytecode_id, (), initial_state, vec![])
        .await;

    let increment = 10u64;
    chain
        .add_block(|block| {
            block.with_operation(application_id, Operation::Increment { value: increment });
        })
        .await;

    let final_value = initial_state + increment;
    let response = chain.graphql_query(application_id, "query { value }").await;
    let state_value = response["value"].as_u64().expect("Failed to get the u64");

    assert_eq!(state_value, final_value);
}
