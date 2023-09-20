solc --bin-runtime \
        --optimize \
        --overwrite \
        --evm-version istanbul \
        --output-dir ../bytecode \
        ../bytecode/Target.sol

solc --bin-runtime \
        --optimize \
        --overwrite \
        --evm-version istanbul \
        --output-dir ../bytecode \
        ../bytecode/Exploiter.sol

cargo run --release -p host