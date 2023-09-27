use fil_actor_eam::Return;
use fil_actor_evm::Method as EvmMethods;
use fil_actors_runtime::EAM_ACTOR_ADDR;
use fvm::executor::{ApplyKind, Executor};
use fvm_integration_tests::dummy::DummyExterns;
use fvm_integration_tests::tester::Account;
use fvm_ipld_encoding::strict_bytes;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::message::Message;
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

use testing::setup;
use testing::GasResult;
use testing::parse_gas;

use testing::types;

use alloy_json_abi::JsonAbi;
use alloy_sol_types::encode_params;
use alloy_primitives;
use alloy_primitives::hex_literal;
use alloy_sol_types::{sol, SolCall, SolStruct};

use hex;
use cbor_data::{CborBuilder, Encoder};

const WASM_COMPILED_PATH: &str = "../build/v0.8/tests/SendApiTest.bin";

#[derive(SerdeSerialize, SerdeDeserialize)]
#[serde(transparent)]
pub struct CreateExternalParams(#[serde(with = "strict_bytes")] pub Vec<u8>);

#[test]
fn send_tests() {
    println!("Testing solidity API");

    let contract_path = "../build/v0.8/tests/SendApiTest.abi";
    let contract_json = std::fs::read_to_string(contract_path).unwrap();

    let contract: JsonAbi = serde_json::from_str(&contract_json).unwrap();

    let mut gas_result: GasResult = vec![];
    let (mut tester, _manifest) = setup::setup_tester();

    // As the governor address for datacap is 200, we create many many address in order to initialize the ID 200 with some tokens
    // and make it a valid address to use.
    let sender: [Account; 300] = tester.create_accounts().unwrap();

    println!(
        "{}",
        format!(
            "Sender address id [{}] and bytes [{}]",
            &sender[0].0,
            hex::encode(&sender[0].1.to_bytes())
        )
    );
    println!(
        "{}",
        format!(
            "Sender address id [{}] and bytes [{}]",
            &sender[1].0,
            hex::encode(&sender[1].1.to_bytes())
        )
    );
    println!(
        "{}",
        format!(
            "Sender address id [{}] and bytes [{}]",
            &sender[2].0,
            hex::encode(&sender[2].1.to_bytes())
        )
    );
    println!(
        "{}",
        format!(
            "Sender address id [{}] and bytes [{}]",
            &sender[3].0,
            hex::encode(&sender[3].1.to_bytes())
        )
    );

    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();

    let executor = tester.executor.as_mut().unwrap();

    // First we deploy the contract in order to actually have an actor running on the embryo address
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

    println!(
        "Contract address ID type on decimal [{}]",
        exec_return.actor_id
    );
    println!(
        "Contract address ID type on hex [{}]",
        hex::encode(Address::new_id(exec_return.actor_id).to_bytes())
    );
    match exec_return.robust_address {
        Some(addr) => println!("Contract address robust type [{}]", addr),
        None => (),
    }

    println!(
        "Contract address eth address type [{}]",
        hex::encode(exec_return.eth_address.0)
    );

    let contract_actor_id = exec_return.actor_id;

    // Send some tokens to the smart contract
    let message = Message {
        from: sender[0].1,
        to: Address::new_id(contract_actor_id),
        gas_limit: 1000000000,
        method_num: 0,
        value: TokenAmount::from_atto(100),
        sequence: 1,
        ..Message::default()
    };

    let res = executor
        .execute_message(message, ApplyKind::Explicit, 100)
        .unwrap();

    assert_eq!(res.msg_receipt.exit_code.value(), 0);

    println!("Calling `send (actor id)`");
    let actor_id = types::FilActorId::from(0);
    let actor_id_data = hex::encode(actor_id.encode_single());

    let amount = types::Amount::from(alloy_primitives::U256::from(0));
    let amount_data = hex::encode(amount.encode_single());

    let selector_actor_id = contract.function("send").unwrap()[0].selector();
    let encoded_selector_actor_id = hex::encode(selector_actor_id);

    let send_fil_actor_id_data = format!("{}{}{}", encoded_selector_actor_id, actor_id_data, amount_data);

    let cbor = CborBuilder::default().encode_array(|builder| {
        builder.encode_bytes(send_fil_actor_id_data);
    });

    let temps = hex::encode_upper(&cbor.as_slice());
    let params = &temps[2..temps.len()];

    let message = Message {
            from: sender[0].1,
            to: Address::new_id(contract_actor_id),
            gas_limit: 1000000000,
            method_num: EvmMethods::InvokeContract as u64,
            sequence: 2,
            params: RawBytes::new(hex::decode(params).unwrap()),
            // params: RawBytes::new(hex::decode("58446F7EE35E0000000000000000000000000000000000000000000000000000000000000065000000000000000000000000000000000000000000000000000000000000000A").unwrap()),
            ..Message::default()
        };

    let res = executor
        .execute_message(message, ApplyKind::Explicit, 100)
        .unwrap();
    let gas_used = parse_gas(res.exec_trace);
    gas_result.push(("send (actor id)".into(), gas_used));
    assert_eq!(res.msg_receipt.exit_code.value(), 0);


    // println!("Calling `send (address)`");

    // let fil_address = types::FilAddress {
    //     data: alloy_primitives::Bytes::from_static(b"01a53e34d73bbd7688a8d8be9448b2ede303349e30").to_vec()
    // };
    // let fil_address_data = hex::encode(encode_params(&(fil_address.tokenize())));

    // let selector = contract.function("send").unwrap()[1].selector();
    // let encoded_selector = hex::encode(selector);

    // let send_fil_address_data = format!("{}{}", encoded_selector, fil_address_data);

    // let cbor = CborBuilder::default().encode_array(|builder| {
    //     builder.encode_bytes(send_fil_address_data);
    // });

    // let temps = hex::encode_upper(&cbor.as_slice());
    // let params = &temps[2..temps.len()];

    // let message = Message {
    //     from: sender[0].1,
    //     to: Address::new_id(contract_actor_id),
    //     gas_limit: 1000000000, method_num: EvmMethods::InvokeContract as u64,
    //     sequence: 3,
    //     params: RawBytes::new(hex::decode(params).unwrap()),
    //     // params: RawBytes::new(hex::decode("58A40E0E687C0000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000A000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000020065000000000000000000000000000000000000000000000000000000000000").unwrap()),
    //     ..Message::default()
    // };

    // let res = executor
    //     .execute_message(message, ApplyKind::Explicit, 100)
    //     .unwrap();
    // let gas_used = parse_gas(res.exec_trace);
    // gas_result.push(("send (address)".into(), gas_used));
    // assert_eq!(res.msg_receipt.exit_code.value(), 0);

    let table = testing::create_gas_table(gas_result);
    testing::save_gas_table(&table, "send");

    table.printstd();
}
