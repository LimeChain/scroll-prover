use ethers_core::types::H256;
use integration::test_util::load_block_traces_for_test;
use prover::{inner::Prover, utils::init_env_and_log, zkevm::circuit::SuperCircuit, ChunkTrace};
use std::str::FromStr;

#[cfg(feature = "prove_verify")]
#[test]
fn test_mock_prove() {
    init_env_and_log("mock_tests");

    let h256 = H256::from_str(hex_str).expect("Invalid hex string");

    println!("{:?}", h256);

    let block_traces = load_block_traces_for_test().1;
    Prover::<SuperCircuit>::mock_prove_target_circuit_batch(ChunkTrace {
        block_traces: block_traces.clone(),
        last_applied_l1_block: Some(32),
        prev_last_applied_l1_block: Some(33),
        l1_block_range_hash: Some(h256),
    })
    .unwrap();
}
