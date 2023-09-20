use methods::{ VERIFY_ELF, VERIFY_ID };
use risc0_zkvm::{ default_executor_from_elf, serde::{ from_slice, to_vec }, ExecutorEnv };

fn main() {
    println!("Proving state change after hack tx...");
    let calldata = "63d9b770";
    let result = run_verifier(calldata);
    println!(
        "Results: there's a certain tx that makes target contract eth balance go [from, to, private input hash] = {:?}",
        result
    );
}

fn run_verifier(calldata: &str) -> Vec<String> {
    println!("Making the prover...");
    let env = ExecutorEnv::builder()
    .add_input(&to_vec(&calldata).unwrap())
    .build()
    .unwrap();

    println!("Running the prover...");

    let mut exec = default_executor_from_elf(env, VERIFY_ELF).unwrap();
    let session = exec.run().unwrap();
    println!("Verifying the prover...");
    let receipt = session.prove().unwrap();
    receipt.verify(VERIFY_ID).unwrap();
    let result: Vec<String> = from_slice(&receipt.journal.as_slice()).unwrap();
    result
}
