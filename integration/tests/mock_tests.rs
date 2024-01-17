use integration::test_util::load_block_traces_for_test;
use prover::{inner::Prover, utils::init_env_and_log, zkevm::circuit::SuperCircuit, ChunkTrace};
use ethers_core::utils::{keccak256, H256};

#[cfg(feature = "prove_verify")]
#[test]
fn test_mock_prove() {
    init_env_and_log("mock_tests");

    let block_traces = load_block_traces_for_test().1;
    Prover::<SuperCircuit>::mock_prove_target_circuit_batch(ChunkTrace {
        block_traces: block_traces.clone(),
        last_applied_l1_block: 32,
        prev_last_applied_l1_block: 33,
        l1_block_range_hash: Some(H256(
            (keccak256(vec!["0xa5566b8827b9e1908989e9e6d3115950fd02deeedd291c6fba18cef10971f14f"])),
        )),
    })
    .unwrap();
}
