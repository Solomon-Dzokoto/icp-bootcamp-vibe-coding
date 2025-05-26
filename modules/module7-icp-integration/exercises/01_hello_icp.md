# Exercise 1: Hello ICP

## Problem Statement

Create a simple greeting canister that runs on the Internet Computer. The canister should store a greeting message and allow users to update and retrieve it.

## Learning Objectives

- Set up a basic Rust canister project for the Internet Computer
- Understand canister state management in Rust
- Implement query and update methods
- Test and deploy a canister locally

## Prerequisites

- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/) installed
- Rust and Cargo installed

## Starter Code

```rust
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk_macros::*;
use std::cell::RefCell;

// TODO: Define a struct to hold the greeting message

// TODO: Create a thread-local variable to store the state

#[init]
fn init() {
    // TODO: Initialize the canister state with a default greeting
}

#[update]
fn update_greeting(new_greeting: String) -> String {
    // TODO: Update the greeting and return the new greeting
}

#[query]
fn get_greeting() -> String {
    // TODO: Return the current greeting
}

// Additional challenge: Add a method to get the greeting history
```

## Expected Project Structure

```
hello_icp/
├── Cargo.toml
├── dfx.json
├── src/
│   └── lib.rs  (containing the canister code)
└── README.md
```

## Steps to Complete

1. Create a new project using the DFX command:

   ```
   dfx new hello_icp --type=rust
   ```

2. Implement the canister code using the starter code above

3. Configure your `dfx.json` to include the Rust canister

4. Deploy and test locally:
   ```
   dfx start --background
   dfx deploy
   dfx canister call hello_icp get_greeting
   dfx canister call hello_icp update_greeting '("Hello, Internet Computer!")'
   ```

## Expected Results

- The canister should store a greeting message (default: "Hello, World!")
- Users should be able to update the greeting with a new message
- Users should be able to query the current greeting

## Tips

- Use `thread_local!` to create state that persists between canister calls
- Remember that all update methods should modify the state through a mutable reference
- Test your canister locally before deploying to the IC mainnet
- Use the Candid UI to interact with your canister during testing
- For the additional challenge, consider using a `Vec<String>` to store the greeting history
