# nn

This is a neural network implementation entirely from scratch as a canister (smart contract) for the ICP blockchain. It trains onchain aswell, I tested it on localnet and it seemed to work pretty well.



If you want to start working on your project right away, you might want to try the following commands:

```bash
cd nn/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
