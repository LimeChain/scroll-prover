use integration::test_util::load_block_traces_for_test;
use prover::{inner::Prover, utils::init_env_and_log, zkevm::circuit::SuperCircuit, ChunkTrace};
use std::str::FromStr;
use ethers_core::abi::Hash;

#[cfg(feature = "prove_verify")]
#[test]
fn test_mock_prove() {
    init_env_and_log("mock_tests");

    let block_traces = load_block_traces_for_test().1;
    Prover::<SuperCircuit>::mock_prove_target_circuit_batch(ChunkTrace {
        block_traces: block_traces.clone(),
        last_applied_l1_block: Some(46),
        prev_last_applied_l1_block: Some(45),
        l1_block_range_hash: Some(Hash::from_str("0xbf6bf5ffabb8af597c2c704c028daa2bd52bb076062961fc912c52c20efc4dad").unwrap()),
    })
    .unwrap();
}
