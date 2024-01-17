use core::slice::SlicePattern;

use ethers_core::{utils::keccak256, types::H256};
use integration::test_util::load_block_traces_for_test;
use prover::{inner::Prover, utils::init_env_and_log, zkevm::circuit::SuperCircuit, ChunkTrace};

#[cfg(feature = "prove_verify")]
#[test]
fn test_mock_prove() {
    init_env_and_log("mock_tests");

    let block_traces = load_block_traces_for_test().1;
    Prover::<SuperCircuit>::mock_prove_target_circuit_batch(ChunkTrace {
        block_traces: block_traces.clone(),
        last_applied_l1_block: Some(32),
        prev_last_applied_l1_block: Some(33),
        l1_block_range_hash: Some(H256("0x3c8e0460c773f86d9170137735eebb0efdb8336362fe98ff0b4b82dd8dff274d".as_bytes().as_slice().try_into().unwrap())),
    })
    .unwrap();
}
