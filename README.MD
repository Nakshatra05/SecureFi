# SecureFi" in brief

"SecureFi" is a Proof-of-Concept. SecureFi at its heart carries the essence which is about securing DeFi Transactions. 
We aim to improvise the connection and collaboration between the Security experts and teams at various DeFi protocols. We aim to do so through the use of Zero-knowledge proofs. 
The core concept behind SecureFi is simple: Generate a zkP for an exploit in a smart contract and report it to the DeFi teams.

# Problem 
Bug bounty programs in the DeFi space face challenges in terms of management, recognition, and compensation for white hat hackers.
This often discourages hackers from reporting vulnerabilities, posing a risk to the overall security of the DeFi ecosystem.

# Our Solution
Our advanced tooling enables auditors to generate a zero-knowledge proof of exploit securely, without disclosing the actual exploit itself. By utilizing zero-knowledge proofs, auditors can demonstrate their knowledge of a transaction that could lead to an undesirable change in contract state, without revealing the specific details of the exploit.
By safeguarding the exploit, auditors foster an environment where project owners are encouraged to collaborate in resolving vulnerabilities. This facilitates effective communication and cooperation between hackers and project owners, promoting a more secure DeFi ecosystem.


# Technologies Used
Our project incorporates the following technologies:

1. [Risc0](https://www.risczero.com/): A versatile Zero-Knowledge Virtual Machine (ZKVM) designed for proving and verifying computations. Risc0 utilizes the RISC Zero ZKVM, which emulates a real embedded RISC-V microprocessor. This empowers developers to write ZK proofs in a manner similar to conventional code development.

2. [SputnikVM](https://github.com/rust-blockchain/evm): A modular and high-performance virtual machine specifically designed for executing Ethereum smart contracts. SputnikVM offers efficient contract execution capabilities within the project's ecosystem.

3. [Zero-Knowledge Proofs](https://en.wikipedia.org/wiki/Zero-knowledge_proof): A cryptographic technique enabling one party to demonstrate the truth of a statement to another party without disclosing any additional information beyond the validity of the statement. Zero-Knowledge Proofs play a crucial role in ensuring privacy and security within the project, as they allow for secure verification of claims without revealing sensitive data.

# Setting up your system
Well, to use the model you will need to have the following installed on your system :
1. [Rust](https://www.rust-lang.org/tools/install)
3. [Solc](https://docs.soliditylang.org/en/v0.8.17/installing-solidity.html)

# Configuring the model

To test the evm :
```bash
$ git clone our File
$ cd SecureFi
$ cd web-gui
$ node main.js
```

after the auditor clicks the audit button to submit his audit, the following script runs to To generate proof for verification : 
```bash
$ solc --bin-runtime \
        --optimize \
        --overwrite \
        --evm-version istanbul \
        --output-dir ../bytecode \
        ../bytecode/Target.sol

$ solc --bin-runtime \
        --optimize \
        --overwrite \
        --evm-version istanbul \
        --output-dir ../bytecode \
        ../bytecode/Exploiter.sol

$ cargo run --release -p host
```
# Challenges Faced 
* Initial configuration of SputnikVM was a tough nut and its operation was computationally expensive too(Minimum 16GB RAM required/32 GB preferred).
* We were digesting information at god's pace. Had to go through a hell lot of documentation for understanding SputnikVM and Just
* The whole idea itself when first came to head seemed challenging since we only had very basic theory-level knowledge of zkPs and a decent knowledge of DeFi.

# References 
* [Run Solidity code inside SputnikVM inside Risc0](https://odra.dev/blog/evm-at-risc0/)
* [zkPoEX developed at ETHDenver](https://github.com/zkoranges/zkPoEX/tree/main)
