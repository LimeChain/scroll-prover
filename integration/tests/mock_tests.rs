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
        last_applied_l1_block: Some(167),
        prev_last_applied_l1_block: Some(166),
        l1_block_range_hash: Some(Hash::from_str("0xb847826b8109c7103096455b71879450d0e1092676fba38b55b44123426601f0").unwrap()),
    })
    .unwrap();
}
