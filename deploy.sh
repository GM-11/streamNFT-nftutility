echo "Deploying Utility Contract..."

soroban contract build
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/utility_contract.wasm \
    --source alice \
    --network testnet

echo "Deployed Utility Contract"
