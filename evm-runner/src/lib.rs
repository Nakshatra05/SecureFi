#![cfg_attr(not(test), no_std)]

extern crate alloc;
extern crate core;

use core::str::FromStr;
use crate::alloc::string::ToString;

use alloc::{ vec::Vec, collections::BTreeMap, string::String, format };

use evm::{
    Config,
    ExitReason,
    ExitSucceed,
    backend::{ MemoryVicinity, MemoryAccount, MemoryBackend },
    executor::stack::{ StackSubstateMetadata, MemoryStackState, StackExecutor },
    Handler,
};

use primitive_types::{ U256, H160, H256 };
use sha3::{ Digest, Keccak256 };
use hex::encode;

pub const TARGET_CONTRACT_EVM_PROGRAM: &str = include_str!("../../bytecode/Target.bin-runtime");
pub const EXPLOITER_CONTRACT_EVM_PROGRAM: &str = include_str!("../../bytecode/Exploiter.bin-runtime");

pub const TARGET_ADDRESS: &str = "0x1000000000000000000000000000000000000000";
pub const EXPLOITER_ADDRESS: &str = "0x2000000000000000000000000000000000000000";
pub const CALLER_ADDRESS: &str = "0xf000000000000000000000000000000000000000";

pub fn run_evm(calldata: &str) -> Vec<String> {
    let config: Config = Config::istanbul();

    let vicinity = MemoryVicinity {
        gas_price: U256::zero(),
        origin: H160::default(),
        block_hashes: Vec::new(),
        block_number: Default::default(),
        block_coinbase: Default::default(),
        block_timestamp: Default::default(),
        block_difficulty: Default::default(),
        block_gas_limit: Default::default(),
        chain_id: U256::one(),
        block_base_fee_per_gas: U256::zero(),
    };

    // chain state
    let mut global_state = BTreeMap::new();
    let target_storage: BTreeMap<H256, H256> = BTreeMap::new();
    let mut exploiter_storage: BTreeMap<H256, H256> = BTreeMap::new();

    // insert target address in exploiter contract
    let target_address = H256::from(H160::from_str(TARGET_ADDRESS).unwrap());

    exploiter_storage.insert(H256::zero(), target_address);

    // formats the exploiter state binary tree map as a string for future hashing

    let mut concat_exploiter_btreemap = String::new();
    concat_exploiter_btreemap += "[";
    for (key, value) in &exploiter_storage {
        concat_exploiter_btreemap += &format!(
            "[{},{}]",
            encode(key.as_bytes()),
            encode(value.as_bytes())
        );
    }
    concat_exploiter_btreemap += "]";
    let exploiter_btreemap = concat_exploiter_btreemap.as_str();

    // deploy target contract to state
    global_state.insert(H160::from_str(TARGET_ADDRESS).unwrap(), MemoryAccount {
        nonce: U256::one(),
        balance: U256::from_dec_str("1000000000000000000").unwrap(), // 1 ether
        storage: target_storage,
        code: hex::decode(TARGET_CONTRACT_EVM_PROGRAM).unwrap(),
    });

    // deploy exploiter contract to state
    global_state.insert(H160::from_str(EXPLOITER_ADDRESS).unwrap(), MemoryAccount {
        nonce: U256::one(),
        balance: U256::from_dec_str("0").unwrap(), // 0 ether
        storage: exploiter_storage,
        code: hex::decode(EXPLOITER_CONTRACT_EVM_PROGRAM).unwrap(),
    });

    // deploy caller address in state
    global_state.insert(H160::from_str(CALLER_ADDRESS).unwrap(), MemoryAccount {
        nonce: U256::one(),
        balance: U256::from_dec_str("10000000000000000000").unwrap(), // 10 ether for the caller
        storage: BTreeMap::new(),
        code: Vec::new(),
    });

    let backend = MemoryBackend::new(&vicinity, global_state);
    let metadata = StackSubstateMetadata::new(u64::MAX, &config);
    let state = MemoryStackState::new(metadata, &backend);
    let precompiles = BTreeMap::new();
    let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);

    let before = executor.balance(H160::from_str(TARGET_ADDRESS).unwrap());

    let (exit_reason, _) = executor.transact_call(
        H160::from_str(CALLER_ADDRESS).unwrap(),
        H160::from_str(EXPLOITER_ADDRESS).unwrap(),
        U256::from_dec_str("1000000000000000000").unwrap(), // 1 ether - 1000000000000000000
        hex::decode(calldata).unwrap(),
        u64::MAX,
        Vec::new()
    );

    assert!(exit_reason == ExitReason::Succeed(ExitSucceed::Stopped));

    let after = executor.balance(H160::from_str(TARGET_ADDRESS).unwrap());

    // hashes private inputs with keccak256
    let mut hasher = Keccak256::new();
    let concatenated: String = format!(
        "[{},{},{},{}]",
        EXPLOITER_CONTRACT_EVM_PROGRAM,
        EXPLOITER_ADDRESS,
        CALLER_ADDRESS,
        exploiter_btreemap
    );

    hasher.update(concatenated);

    let hash = hasher.finalize();
    let hash_str = hex::encode(hash.to_vec());

    // constraint: caller address, target address and exploiter address are different
    assert!(CALLER_ADDRESS != TARGET_ADDRESS);
    assert!(CALLER_ADDRESS != EXPLOITER_ADDRESS);
    assert!(TARGET_ADDRESS != EXPLOITER_ADDRESS);

    // simulataion outputs: the before and after hack balance of ETH of the target
    let mut outputs = Vec::new();

    outputs.push(before.to_string());
    outputs.push(after.to_string());
    outputs.push(hash_str);

    outputs
}
