# Liquidity Transformer - Casper Blockchain

Implementation of `Synthetic CSPR`, `Synthetic Helper`, `Synthetic Token` and `Liquidity Transformer` for the CasperLabs platform.

## NOTE:- Following repositories are required to place with this project also please make sure names of the repositories should be same as in make file
## NOTE:- Investment days can be adjusted by changing the constant value 'pub const INVESTMENT_DAYS: u8 = 15;' in the following file (./CasperLabs-Wise-LiquidityTransformer/liquidity_transformer/liquidity_transformer_crate/src/data.rs)

1. [Uniswap core contracts](https://github.com/Rengo-Labs/CasperLabs-UniswapV2-Core)
2. [Uniswap router contracts](https://github.com/Rengo-Labs/CasperLabs-UniswapV2-Router)
3. [Stakeable token wise contracts](https://github.com/Rengo-Labs/CasperLabs-StakeableToken)

## Steps

There are 2 contracts in this folder

1. Liquidity Transformer Contract
2. Synthetic CSPR Contract

There and 2 crates in this folder

3. Synthetic Helper Crate
4. Synthetic Token Crate

## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Install the prerequisites](#install-the-prerequisites)
  - [Creating Keys](#creating-keys)
  - [Usage](#usage)
    - [Install](#install)
    - [Build Individual Smart Contract](#build-individual-smart-contract)
    - [Build All Smart Contracts](#build-all-smart-contracts)
    - [Individual Test Cases](#individual-test-cases)
    - [All Test Cases](#all-test-cases)
  - [Known contract hashes](#known-contract-hashes)

### Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

```bash
# Update package repositories
sudo apt update
# Install the command-line JSON processor
sudo apt install jq -y
# Install rust
# Choose cutomize intallation to install nightly version
# Install the nightly version (by default stable toolchain is installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
# Check that nightly toolchain version is installed(this will list stable and nightly versions)
rustup toolchain list
# Set rust nightly as default
rustup default nightly
# Install wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
# Rust Version
rustup --version
# Install Cmake
sudo apt-get -y install cmake
Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html
# cmake Version
cmake --version
# Installing the Casper Crates
cargo install cargo-casper
# Add Casper repository
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
sudo apt-key add casper-repo-pubkey.asc
sudo apt update
sudo apt install libssl-dev
sudo apt install pkg-config
# Install the Casper client software
cargo +nightly install casper-client
# To check Casper Client Version
casper-client --version
# Commands for help
casper-client --help
casper-client <command> --help
```

### Creating Keys

```bash
# Create keys
casper-client keygen <TARGET DIRECTORY>
```

#### Install

Make sure `wasm32-unknown-unknown` is installed.

```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

### NOTE:- !IMPORTANT! 'make prepare' command should also be excuted seperately for 'CasperLabs-Wise-StakeableToken' by going in the contract folder and execute above command

#### Build Smart Contract

Run this command to build Smart Contract.

```
make build-contract
```

#### Build All Smart Contracts

Run this command in main folder to build all Smart Contracts.

```
make build-all
```

#### Run individual Test Cases

Run this command to run Test Cases.

```
make test-<CONTRACT-NAME>
```

#### Run all Test Cases

Run this command in main folder to run all contract's Test Cases.

```
make test
```

#### Build & Test all contracts

Run this command in main folder to build all contract and dependent contracts and run test cases.

```
make test-all
```

### Deploying Liquidity Transformer contract manually

If you need to deploy the `Liquidity Transformer contract` manually you need to pass the some parameters. Following is the command to deploy the `Liquidity Transformer contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="wise_token:Key='wise-contract-hash'" \
    --session-arg="scspr:Key='scspr-hash'" \
    --session-arg="uniswap_pair:Key='uniswap-pair-hash'" \
    --session-arg="uniswap_router:Key='uniswap-router-hash'" \
    --session-arg="wcspr:Key='wcspr-hash'" \
    --session-arg="amount:u512='payable-amount'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="LiquidityTransformer-entry-point-methods"></a>

Following are the LiquidityTransformer's entry point methods.

- #### set_settings <a id="LiquidityTransformer-set-settings"></a>
  Keeper to set address of wise, scspr, uniswap_pair.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| wise_token     | Key  |
| uniswap_pair   | Key  |
| synthetic_cspr | Key  |

This method **returns** nothing.

- #### renounce_keeper <a id="LiquidityTransformer-renounce-keeper"></a>
  Keeper to renounce its keeper status.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### reserve_wise <a id="LiquidityTransformer-reserve-wise"></a>
  Used to reserve wise by sending value to be deducted from caller_purse.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| investment_mode | u8   |
| msg_value       | U256 |
| caller_purse    | URef |

This method **returns** nothing.

- #### reserve_wise_with_token <a id="LiquidityTransformer-reserve-wise-with-token"></a>
  Used to reserve wise by sending token from which value will be deducted.

Following is the table of parameters.

| Parameter Name  | Type |
| --------------- | ---- |
| token_address   | Key  |
| token_amount    | U256 |
| investment_mode | u8   |
| caller_purse    | URef |

This method **returns** nothing.

- #### forward_liquidity <a id="LiquidityTransformer-forward-liquidity"></a>
  This method will forward the liquidity after investment days by using uniswap router add liquidity.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| pair           | Key  |

This method **returns** nothing.

- #### get_my_tokens <a id="LiquidityTransformer-get-my-tokens"></a>
  Gets the tokens by mint_supply of wise contract to the `self.get_caller()`

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### payout_investor_address <a id="LiquidityTransformer-payout-investor-address"></a>
  mint_supply of wise contract to the `investor_address`

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| investor_address | Key  |

This method **returns** U256.

- #### prepare_path <a id="LiquidityTransformer-prepare-path"></a>
  Prepare the path of `token_address` and `wcspr`

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| token_address  | Key  |

This method **returns** `Vec<Key>`.

- #### current_stakeable_day <a id="LiquidityTransformer-current-stakeable-day "></a>
  Checks the current wise day value

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** u64.

- #### request_refund <a id="LiquidityTransformer-request-refund"></a>
  Request the refund to the caller_purse send from the caller.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| caller_purse   | URef |

This method **returns** Tuple2(U256, U256).

- #### fund_contract <a id="LiquidityTransformer-fund-contract"></a>
  Allows to deposit fund to contract.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| purse          | URef |
| amount         | U512 |

This method **returns** nothing.

- #### contract_read_only_purse <a id="LiquidityTransformer-contract-read-only-purse"></a>
  Provide 'READ-ADD' contract purse so it can be used in set_liquidity_transformer or can be used to fund contract bu sending this purse.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** URef.

### Deploying SCSPR contract manually

If you need to deploy the `Synthetic CSPR` manually you need to pass the some parameters. Following is the command to deploy the `scspr`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="wcspr:Key='wcspr-hash'" \
    --session-arg="uniswap_pair:Key='uniswap-pair-hash'" \
    --session-arg="uniswap_router:Key='uniswap-router-hash'" \
    --session-arg="uniswap_factory:Key='uniswap-factory-hash'" \
    --session-arg="amount:u512='payable-amount'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="Scspr-entry-point-methods"></a>

Following are the SCSPR's entry point methods.

- #### set_master <a id="Scspr-set-master"></a>
  To set the master address of the contract.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| master_address | Key  |

This method **returns** nothing.

- #### set_wise <a id="Scspr-set-wise"></a>
  To set the wise contract address.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| wise           | Key  |

This method **returns** nothing.

- #### deposit <a id="Scspr-deposit"></a>
  To deposit amount into the scspr contract

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |
| purse          | URef |

This method **returns** nothing.

- #### withdraw <a id="Scspr-withdraw"></a>
  To withdraw amount from the scspr contract

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |
| purse          | URef |

This method **returns** nothing.

- #### liquidity_deposit <a id="Scspr-liquidity-deposit"></a>
  To mint tokens to the caller address

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |
| purse          | URef |

This method **returns** nothing.

- #### form_liquidity <a id="Scspr-form-liquidity"></a>
  Creates initial liquidity on uniswap by forwarding reserved tokens equivalent to CSPR contributed to the contract

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| pair           | Key  |

This method **returns** U256.

- #### renounce_ownership <a id="Scspr-renounce-ownership"></a>
  To renounce ownership to zero address

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### forward_ownership <a id="Scspr-forward-ownership"></a>
  To forward ownership to new_master

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| new_master     | Key  |

This method **returns** nothing.

- #### add_lp_tokens <a id="Scspr-add-lp-tokens"></a>
  To deposit value from contract and transfer pair amount to this contract address

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| purse          | URef |
| amount         | U256 |
| token_amount   | U256 |

This method **returns** nothing.

- #### define_token <a id="Scspr-define-token"></a>
  To define token and make set_token_defined true

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| wise_token     | Key  |

This method **returns** Key.

- #### define_helper <a id="Scspr-define-helper"></a>
  To define transfer helper and make set_helper_defined true

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| transfer_helper  | Key  |

This method **returns** Key.

- #### create_pair <a id="Scspr-create-pair"></a>
  To create_pair by calling factory create pair and make wcspr & scspr pair

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| pair             | Key  |

This method **returns** nothing.

- #### mint <a id="Scspr-mint"></a>
  This function is to mint token against the address that user provided

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| recipient        | Key  |
| amount           | U256 |

This method **returns** nothing.

- #### approve <a id="Scspr-approve"></a>
  This function is to approve tokens against the address that user provided

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| spender          | Key  |
| amount           | U256 |

This method **returns** nothing.

- #### transfer <a id="Scspr-transfer"></a>
  This function is to transfer tokens against the address that user provided

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| recipient        | Key  |
| amount           | U256 |

This method **returns** Result<(), u32>.

- #### transfer_from <a id="Scspr-transfer-from"></a>
  This function is to transfer tokens against the address that has been approved before by owner

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| owner            | Key  |
| recipient        | Key  |
| amount           | U256 |

This method **returns** Result<(), u32>.

- #### balance_of <a id="Scspr-balance-of"></a>
  This function is to return the Balance  of owner against the address that user provided

Following is the table of parameters.

| Parameter Name   | Type |
| ---------------- | ---- |
| owner            | Key  |

This method **returns** U256.

- #### fund_contract <a id="Scspr-fund-contract"></a>
  Allows to deposit fund to contract.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| purse          | URef |
| amount         | U512 |

This method **returns** nothing.

- #### wcspr <a id="Scspr-wcspr"></a>
  Gives address of stored wcspr

Following is the table of parameters.

| Parameter Name      | Type |
| ------------------- | ---- |

This method **returns** Key.

- #### uniswap_router <a id="Scspr-uniswap-router"></a>
  Gives address of stored uniswap router

Following is the table of parameters.

| Parameter Name      | Type |
| ------------------- | ---- |

This method **returns** Key.

- #### uniswap_router <a id="Scspr-uniswap-pair"></a>
  Gives address of stored uniswap pair

Following is the table of parameters.

| Parameter Name      | Type |
| ------------------- | ---- |

This method **returns** Key.

- #### get_trading_fee_amount <a id="Scspr-get-trading-fee-amount"></a>
  Provides current calculated trading fee amount

Following is the table of parameters.

| Parameter Name      | Type |
| ------------------- | ---- |
| previous_evaluation | U256 |
| current_evaluation  | U256 |

This method **returns** U256.

- #### get_amount_payout <a id="Scspr-get-amount-payout"></a>
  Gives the payout amount

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |

This method **returns** U256.

- #### get_synthetic_balance <a id="Scspr-get-synthetic-balance"></a>
  Gives the synthetic balance

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### get_wrapped_balance <a id="Scspr-get-wrapped-balance"></a>
  Gives the wrapped balance

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### get_evaluation <a id="Scspr-get-evaluation"></a>
  Gives the amount of evaluation after calculation

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### get_pair_balances <a id="Scspr-get-pair-balances"></a>
  Gives the pair balances

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Tuple2(U256, U256).

- #### get_lp_token_balance <a id="Scspr-get-lp-token-balance"></a>
  Gives the lp token balance

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### get_liquidity_percent <a id="Scspr-get-liquidity-percent"></a>
  Gives the percentage of the liquidity

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### master_address <a id="Scspr-master-address"></a>
  Gives the master address

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Key.

- #### current_evaluation <a id="Scspr-current-evaluation"></a>
  Gives the current evaluation

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Key.

- #### transfer_helper <a id="Scspr-transfer-helper"></a>
  Gives the transfer helper

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Key.

- #### token_defined <a id="Scspr-token-defined"></a>
  Gives the defined token status

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** bool.

- #### allow_deposit <a id="Scspr-allow-deposit"></a>
  Gives the deposit allow status

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** bool.

- #### helper_defined <a id="Scspr-helper-defined "></a>
  Gives the helper defined status

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** bool.

- #### bypass_enabled <a id="Scspr-bypass-enabled"></a>
  Gives the bypass enables status

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** bool.
