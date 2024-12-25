# Blockchain-Powered Motorcycle Loan Platform

This project is a Web3-based platform built to enable transparent, secure, and automated loan agreements for motorcycle financing. The system leverages the Internet Computer's capabilities for blockchain-powered smart contract interactions, ensuring a seamless experience for borrowers, lenders, and investors.

## Features

- **User Management**: Register and manage borrowers, investors, and admins.
- **Motorcycle Management**: Add and track motorcycle details including availability and status.
- **Loan Management**: Create, update, and manage loans for motorcycle financing.
- **Payment Processing**: Log and manage payments for loans with real-time updates.
- **Investor Management**: Register investors and manage investments in loan pools.
- **Loan Pool Management**: Create and allocate loan pools to streamline lending and funding.
## Requirements

- rustc 1.64 or higher

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```

- rust wasm32-unknown-unknown targetz

```bash
$ rustup target add wasm32-unknown-unknown
```

- candid-extractor

```bash
$ cargo install candid-extractor
```

- install `dfx`

```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ git clone https://github.com/aarontsimamgo/inter-uni-sports-league.git
$ cd inter-uni-sports-league/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:

```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:

```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background --clean

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
