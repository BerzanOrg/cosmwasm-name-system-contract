# cosmwasm-name-system-contract

This repository contains a Rust project developed for exploring [CosmWasm](https://cosmwasm.com/).

The smart contract acts as a name registry. Users can register and unregister names.

The tests are in [`tests/`](https://github.com/BerzanXYZ/cosmwasm-name-system-contract/tree/main/tests) file.

Generated JSON schema is in [`schema/`](https://github.com/BerzanXYZ/cosmwasm-name-system-contract/tree/main/schema) folder.


## Developing
Clone the repository:
```sh
git clone https://github.com/berzanxyz/cosmwasm-name-system-contract.git
```

Set current directory:
```sh
cd cosmwasm-name-system-contract/
```

Start VS Code:
```sh
code .
```

Reopen the folder in a container:

> VS Code will notify you to reopen the folder in a container. 
>
> Make sure you have Docker installed. 



## Building the smart contract
```sh
cargo wasm-relase # or cargo wasm-debug  
```

## Running tests 
```sh
cargo test
```

## Generating JSON schema 
```sh
cargo schema 
```

## Checking if the smart contract is valid
```sh
cosmwasm-check target/wasm32-unknown-unknown/release/cosmwasm_name_system_contract.wasm 
```