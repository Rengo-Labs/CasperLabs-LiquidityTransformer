# Liquidity Transformer - Casper Blockchain

Implementation of `Synthetic CSPR`, `Synthetic Helper`, `Synthetic Token` and `Liquidity Transformer` Contract for the CasperLabs platform.
d
## NOTE:- Following repositories are required to be placed with this project in the same root directory and names of the repositories should be same as in the "Makefile"

1. [Uniswap Core Contracts](https://github.com/Scytalelabs-official/CasperLabs-UniswapV2-core)
2. [Uniswap Router Contracts](https://github.com/Scytalelabs-official/Casperlabs-UniswapRouter)
3. [Stakeable Token Wise Contracts](https://github.com/Scytalelabs-official/CasperLabs-Wise-StakeableToken)

## Steps

There are 4 contracts in this folder

1. Synthetic CSPR Contract
2. Synthetic Helper Contract
3. Synthetic Token Contract
4. Liquidity Transformer

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

**Note:** If any command fails try agin by restart the terminal to reset the enviornment variable.

```bash
# Update package repositories
sudo apt update
# Install the command-line JSON processor
sudo apt install jq -y
# Install rust
# Install the nightly version (by default stable toolchain is installed)
# And proceed with the installation (default)
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
casper-client keygen <TARGET DIRECTORY NAME>
```
**Note:** Choose the name of directory where generated keys will be stored.


#### NOTE

Make sure `wasm32-unknown-unknown `is installed.

```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

#### Build Smart Contract

Run this command to build Smart Contract of the Liquidity Transformer Repository.

```
make build-contract
```

#### Build All Smart Contracts

Run this command in root folder to build all Smart Contracts of Uniswap Core, Uniswap Router, Stakeable Token contracts and Liquidity Transformer .

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

Run this command in main folder to run all contract's Test Cases.

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
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="LiquidityTransformer-entry-point-methods"></a>

Following are the LiquidityTransformer's entry point methods.

- #### set_settings <a id="LiquidityTransformer-set-settings"></a>
  Only keeper can set wise, scspr, uniswap_pair.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| wise_token     | Key  |
| uniswap_pair   | Key  |
| synthetic_cspr | Key  |

This method **returns** nothing.

- #### renounce_keeper <a id="LiquidityTransformer-renounce-keeper"></a>
  Only keeper can set keeper to zero address.

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

- #### current_wise_day <a id="LiquidityTransformer-current-wise-day "></a>
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

### Deploying SYNTHETIC HELPER contract manually

If you need to deploy the `SYNTHETIC HELPER contract` manually you need to pass the hashes of the other contracts as parameter. Following is the command to deploy the `SYNTHETIC HELPER contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
```

## Entry Point methods <a id="SyntheticHelper-entry-point-methods"></a>

Following are the SYNTHETIC HELPER's entry point methods.

- #### prepare_path <a id="SyntheticHelper-prepare-path"></a>
  Prepare two given tokens path

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| token_from     | Key  |
| token_to       | Key  |

This method **returns** nothing.

- #### get_double_root <a id="SyntheticHelper-get-double-root"></a>
  Doubles the amount given

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |

This method **returns** U256.

- #### swap <a id="SyntheticHelper-get-balance-half"></a>
  Gets the half balance.

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U512.

- #### get_balance_diff <a id="SyntheticHelper-get-balance-diff"></a>
  Calculates the difference of caller purse balance and sent parameter

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |

This method **returns** U512.

- #### get_balance_of <a id="SyntheticHelper-get-balance-of"></a>
  Get balance of the token owner

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| token          | Key  |
| owner          | Key  |

This method **returns** U256.

- #### fund_contract <a id="SyntheticHelper-fund-contract"></a>
  Funds the contract from the caller purse

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U512 |

This method **returns** nothing.

### Deploying SYNTHETIC TOKEN contract manually

If you need to deploy the `SYNTHETIC TOKEN contract` manually you need to pass the some parameters. Following is the command to deploy the `SYNTHETIC TOKEN contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="wcspr:Key='wcspr-hash'" \
    --session-arg="synthetic_helper:Key='scspr-hash'" \
    --session-arg="uniswap_pair:Key='pair-hash'" \
    --session-arg="uniswap_router:Key='router-hash'" \
    --session-arg="erc20:Key='erc20-hash'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="SyntheticToken-entry-point-methods"></a>

Following are the SYNTHETIC TOKEN's entry point methods.

- #### set_scspr <a id="SyntheticToken-set-scspr"></a>
  scspr setter

| Parameter Name | Type |
| -------------- | ---- |
| scspr          | Key  |

This method **returns** nothing.

- #### master_address <a id="SyntheticToken-master-address"></a>
  Gives the master address

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Key.

- #### set_master_address <a id="SyntheticToken-set-master-address"></a>
  sets the master address

| Parameter Name | Type |
| -------------- | ---- |
| master_address | Key  |

This method **returns** nothing.

- #### allow_deposit <a id="SyntheticToken-allow-deposit"></a>
  Gives the deposit allow status

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** bool.

- #### set_allow_deposit <a id="SyntheticToken-set-allow-deposit"></a>
  Sets allow deposit status

| Parameter Name | Type |
| -------------- | ---- |
| allow_deposit  | bool |

This method **returns** nothing.

- #### token_defined <a id="SyntheticToken-token-defined"></a>
  Gives the defined token status

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** bool.

- #### set_token_defined <a id="SyntheticToken-set-token-defined"></a>
  Set the token defined

| Parameter Name | Type |
| -------------- | ---- |
| token_defined  | bool |

This method **returns** nothing.

- #### helper_defined <a id="SyntheticToken-helper-defined "></a>
  Gives the helper defined status

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** bool.

- #### set_helper_defined <a id="SyntheticToken-set-helper-defined"></a>
  Set the helper defined status

| Parameter Name | Type |
| -------------- | ---- |
| helper_defined | bool |

This method **returns** nothing.

- #### uniswap_pair <a id="SyntheticToken-uniswap-pair"></a>
  Gives the uniswap paur hash

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Key.

- #### set_uniswap_pair <a id="SyntheticToken-set-uniswap-pair"></a>
  Set the uniswap pair hash

| Parameter Name | Type |
| -------------- | ---- |
| uniswap_pair   | Key  |

This method **returns** nothing.

- #### uniswap_router <a id="SyntheticToken-uniswap-router"></a>
  Gives the router hash

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Key.

- #### set_uniswap_router <a id="SyntheticToken-set-uniswap-router"></a>
  Set the uniswap router hash

| Parameter Name | Type |
| -------------- | ---- |
| uniswap_router | Key  |

This method **returns** nothing.

- #### limit_amount <a id="SyntheticToken-limit-amount"></a>
  Gives the limit amount

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### limit_amount <a id="SyntheticToken-wcspr"></a>
  Gives the wcspr hash

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Key.

- #### bypass_enabled <a id="SyntheticToken-bypass-enabled"></a>
  Gives the bypass enables status

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** bool.

- #### get_trading_fee_amount <a id="SyntheticToken-get-trading-fee-amount"></a>
  Gives the wcspr hash

Following is the table of parameters.

| Parameter Name      | Type |
| ------------------- | ---- |
| previous_evaluation | U256 |
| current_evaluation  | U256 |

This method **returns** U256.

- #### get_amount_payout <a id="SyntheticToken-get-amount-payout"></a>
  Gives the payout amount

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |

This method **returns** U256.

- #### get_wrapped_balance <a id="SyntheticToken-get-wrapped-balance"></a>
  Gives the wrapped balance

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### get_synthetic_balance <a id="SyntheticToken-get-synthetic-balance"></a>
  Gives the synthetic balance

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### get_evaluation <a id="SyntheticToken-get-evaluation"></a>
  Gives the amount of evaluation

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### get_pair_balances <a id="SyntheticToken-get-pair-balances"></a>
  Gives the pair balances

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** Tuple2(U256, U256).

- #### get_lp_token_balance <a id="SyntheticToken-get-lp-token-balance"></a>
  Gives the lp token balance

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### get_liquidity_percent <a id="SyntheticToken-get-liquidity-percent"></a>
  Gives the percentage of the liquidity

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### fees_decision <a id="SyntheticToken-fees-decision"></a>
  Used for the decision of fees

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### extract_and_send_fees <a id="SyntheticToken-extract-and-send-fees"></a>
  Used for the decision of fees

Following is the table of parameters.

| Parameter Name      | Type |
| ------------------- | ---- |
| previous_evaluation | U256 |
| current_evaluation  | U256 |

This method **returns** nothing.

- #### swap_exact_tokens_for_tokens <a id="SyntheticToken-swap-exact-tokens-for-tokens"></a>
  Performs swapping of exact tokens for tokens

Following is the table of parameters.

| Parameter Name     | Type |
| ------------------ | ---- |
| amount             | U256 |
| amount_out_min     | U256 |
| from_token_address | Key  |
| to_token_address   | Key  |

This method **returns** nothing.

- #### add_liquidity <a id="SyntheticToken-add-liquidity"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| \_amount_wcspr | U256 |
| \_amount_scspr | U256 |

This method **returns** Tuple2(U256, U256).

- #### remove_liquidity <a id="SyntheticToken-remove-liquidity"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount         | U256 |

This method **returns** Tuple2(U256, U256).

- #### profit_arbitrage_remove <a id="SyntheticToken-profit-arbitrage-remove"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### to_remove_cspr <a id="SyntheticToken-to-remove-cspr"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### swap_amount_arbitrage_scspr <a id="SyntheticToken-swap-amount-arbitrage-scspr"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** U256.

- #### self_burn <a id="SyntheticToken-self-burn"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### clean_up <a id="SyntheticToken-clean-up"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| deposit_amount | U256 |

This method **returns** nothing.

- #### unwrap <a id="SyntheticToken-unwrap"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount_wcspr   | U256 |

This method **returns** nothing.

- #### profit <a id="SyntheticToken-profit"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |
| amount_wcspr   | U256 |

This method **returns** nothing.

- #### update_evaluation <a id="SyntheticToken-update-evaluation"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### skim_pair <a id="SyntheticToken-skim-pair"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### arbitrage_decision <a id="SyntheticToken-arbitrage-decision"></a>

Following is the table of parameters.

| Parameter Name | Type |
| -------------- | ---- |

This method **returns** nothing.

- #### arbitrage_scspr <a id="SyntheticToken-arbitrage-scspr"></a>

Following is the table of parameters.

| Parameter Name    | Type |
| ----------------- | ---- |
| wrapped_balance   | U256 |
| synthetic_balance | U256 |

This method **returns** nothing.

- #### arbitrage_cspr <a id="SyntheticToken-arbitrage-cspr"></a>

Following is the table of parameters.

| Parameter Name    | Type |
| ----------------- | ---- |
| wrapped_balance   | U256 |
| synthetic_balance | U256 |

This method **returns** nothing.
