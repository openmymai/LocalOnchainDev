## Local Onchain Development

- To develop onchain programs locally, you need the Solana CLI, Rust, and (optional, but recommended) Anchor.
- You can use anchor init to create a new blank Anchor project.
- anchor test runs your tests and also builds your code.

##### Download the Solana CLI Tool
```
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

##### Running the Solana Test Validator
```
solana-test-validator
```

##### Download Anchor
```
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
```
##### Then proceed...
```
avm install latest
avm use latest
```

##### Verify your Anchor Installation
```
anchor init temp-project
cd temp-project
anchor test
```
