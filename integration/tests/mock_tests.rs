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
        last_applied_l1_block: Some(33),
        prev_last_applied_l1_block: Some(32),
        l1_block_range_hash: Some(Hash::from_str("90d8dd7cdea5fadd7b10c681f5e5b520772b4b843705532465f896049d48b058").unwrap()),
    })
    .unwrap();
}
