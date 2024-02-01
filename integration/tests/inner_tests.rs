use integration::test_util::{load_block_traces_for_test, PARAMS_DIR};
use prover::{
    inner::{Prover, Verifier},
    utils::init_env_and_log,
    zkevm::circuit::SuperCircuit,
    ChunkTrace,
};
use ethers_core::abi::Hash;
use std::str::FromStr;

#[cfg(feature = "prove_verify")]
#[test]
fn test_inner_prove_verify() {
    let test_name = "inner_tests";
    let output_dir = init_env_and_log(test_name);
    log::info!("Initialized ENV and created output-dir {output_dir}");

    let block_traces = load_block_traces_for_test().1;
    log::info!("Loaded chunk trace");

    let mut prover = Prover::<SuperCircuit>::from_params_dir(PARAMS_DIR);
    log::info!("Constructed prover");

    let proof = prover
        .load_or_gen_inner_proof(
            test_name,
            "inner",
            ChunkTrace {
                block_traces: block_traces.clone(),
                last_applied_l1_block: Some(46),
                prev_last_applied_l1_block: Some(45),
                l1_block_range_hash: Some(Hash::from_str("0xbf6bf5ffabb8af597c2c704c028daa2bd52bb076062961fc912c52c20efc4dad").unwrap()),
            },
            Some(&output_dir),
        )
        .unwrap();
    log::info!("Got inner snark");

    let verifier = Verifier::<SuperCircuit>::from_params_dir(PARAMS_DIR, None);
    assert!(verifier.verify_inner_snark(proof.to_snark()));
    log::info!("Finish inner snark verification");
}
