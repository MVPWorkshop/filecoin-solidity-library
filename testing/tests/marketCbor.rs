use fil_actor_eam::Return;
use fil_actor_evm::Method as EvmMethods;
use fil_actors_runtime::EAM_ACTOR_ADDR;
use fvm::executor::{ApplyKind, Executor};
use fvm_integration_tests::dummy::DummyExterns;
use fvm_integration_tests::tester::Account;
use fvm_ipld_encoding::strict_bytes;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::address::Address;
use fvm_shared::message::Message;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use alloy_primitives::{fixed_bytes};
use alloy_sol_types::{SolCall};

use testing::{setup, api_contracts};

const WASM_COMPILED_PATH: &str = "../build/v0.8/tests/MarketCBORTest.bin";

#[derive(SerdeSerialize, SerdeDeserialize)]
#[serde(transparent)]
pub struct CreateExternalParams(#[serde(with = "strict_bytes")] pub Vec<u8>);

#[test]
fn market_cbor_tests() {
    println!("Testing Market cbor lib");

    let (mut tester, _manifest) = setup::setup_tester();

    let sender: [Account; 1] = tester.create_accounts().unwrap();

    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();

    let executor = tester.executor.as_mut().unwrap();

    println!("Calling init actor (EVM)");

    let evm_bin = setup::load_evm(WASM_COMPILED_PATH);

    let constructor_params = CreateExternalParams(evm_bin);

    let message = Message {
        from: sender[0].1,
        to: EAM_ACTOR_ADDR,
        gas_limit: 1000000000,
        method_num: 4,
        sequence: 0,
        params: RawBytes::serialize(constructor_params).unwrap(),
        ..Message::default()
    };

    let res = executor
        .execute_message(message, ApplyKind::Explicit, 100)
        .unwrap();

    assert_eq!(res.msg_receipt.exit_code.value(), 0);

    let exec_return: Return = RawBytes::deserialize(&res.msg_receipt.return_data).unwrap();

    let contract_actor_id = exec_return.actor_id;

    println!("Calling `testDealProposalSerDes`");

    let abi_encoded_call = api_contracts::market_cbor_test::testDealProposalSerDesCall{}.abi_encode();

    let cbor_encoded = api_contracts::cbor_encode(abi_encoded_call);

    let message = Message {
            from: sender[0].1,
            to: Address::new_id(contract_actor_id),
            gas_limit: 1000000000,
            method_num: EvmMethods::InvokeContract as u64,
            sequence: 1,
            params: RawBytes::new(hex::decode(
                // "445945a4e4"
                cbor_encoded.as_str()
            ).unwrap()),
            ..Message::default()
        };

    let res = executor
        .execute_message(message, ApplyKind::Explicit, 100)
        .unwrap();

    assert_eq!(res.msg_receipt.exit_code.value(), 0);
}
