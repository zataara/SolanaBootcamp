# Homework 8

## Solana Tokens

### Instructions
Follow the instructions from the lesson and use the `spl-token-cli` to create the following:
1. A fungible token with a supply of 10,000.
2. An NFT.

### Tasks
1. Try sending these tokens to others in your team, and use the command line to find details about the tokens.
2. Try sending using both the `transfer` and `transfer --fund-recipient` approaches.

## Solana Programs

### Using the examples in the repo
1. Modify the existing `msg!` in `example1-helloworld` to log the program ID.
2. Retrieve the total program size of `example1-helloworld`.
3. Retrieve the lamport balance of `example2-counter`.
4. Modify the client for `example2-counter` to feed an incorrect address for the greeting account to the program.

### Hints
Use `solana account <ADDRESS>` to find out more about a given address content.

### Repository
[Solana Bootcamp Examples](https://github.com/ExtropyIO/SolanaBootcamp/tree/main/examples_baremetal)
