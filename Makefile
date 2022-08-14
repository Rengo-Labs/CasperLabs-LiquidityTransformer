uniswap_core_directory = ../CasperLabs-UniswapV2-core
uniswap_router_directory = ../Casperlabs-UniswapRouter
stakeable_token_directory = ../CasperLabs-Wise-StakeableToken
liquidity_transformer_directory = .

# Core Contracts
erc20_contract = ${uniswap_core_directory}/erc20/
factory_contract = ${uniswap_core_directory}/factory/
flash_swapper_contract = ${uniswap_core_directory}/flashswapper/
pair_contract = ${uniswap_core_directory}/pair/
wcspr_contract = ${uniswap_core_directory}/wcspr/

# Router Contracts
library_contract = ${uniswap_router_directory}/uniswap-v2-library/
router_contract = ${uniswap_router_directory}/uniswap-v2-router/

# Wise Contracts
stakeable_token_contract = ${stakeable_token_directory}/stakeabletoken/
liquidity_guard_contract = ${stakeable_token_directory}/liquidity_guard/
transfer_helper_contract = ${stakeable_token_directory}/transfer_helper/

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_liquidity_transformer_path = ${liquidity_transformer_directory}/liquidity_transformer/liquidity_transformer_tests/wasm/
wasm_dest_scspr_path = ${liquidity_transformer_directory}/scspr/scspr_tests/wasm/
wasm_dest_synthetic_token_path = ${liquidity_transformer_directory}/synthetic_token/synthetic_token_tests/wasm/

prepare:
	rustup target add wasm32-unknown-unknown

build-all:
    # Building contracts
	cd ${erc20_contract} && make build-contract
	cd ${factory_contract} && make build-contract
	cd ${flash_swapper_contract} && make build-contract
	cd ${pair_contract} && make build-contract
	cd ${wcspr_contract} && make build-contract
	cd ${library_contract} && make build-contract
	cd ${router_contract} && make build-contract
	cd ${stakeable_token_contract} && make build-contract
	cd ${liquidity_guard_contract} && make build-contract
	cd ${transfer_helper_contract} && make build-contract
	cd ${liquidity_transformer_directory} && make build-contract

    # Copying wasm files
	make copy-wasm-file

clean-all:
    # Cleaning contracts
	make clean
	cd ${erc20_contract} && make clean
	cd ${factory_contract} && make clean
	cd ${flash_swapper_contract} && make clean
	cd ${pair_contract} && make clean
	cd ${wcspr_contract} && make clean
	cd ${library_contract} && make clean
	cd ${router_contract} && make clean
	cd ${stakeable_token_contract} && make clean
	cd ${liquidity_guard_contract} && make clean
	cd ${transfer_helper_contract} && make clean
	cd ${liquidity_transformer_directory} && make clean

# Copying wasms to required directory
copy-wasm-file:
    # Liquidity transformer
	cp ${erc20_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${factory_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${flash_swapper_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${pair_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${router_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${library_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${stakeable_token_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${transfer_helper_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${liquidity_guard_contract}${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
	cp ${liquidity_transformer_directory}/${wasm_src_path}*.wasm ${wasm_dest_liquidity_transformer_path}
    # Scspr
	cp ${erc20_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${factory_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${flash_swapper_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${pair_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${router_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${library_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${stakeable_token_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${liquidity_guard_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${transfer_helper_contract}${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
	cp ${liquidity_transformer_directory}/${wasm_src_path}*.wasm ${wasm_dest_scspr_path}
    # Synthetic Token
	cp ${factory_contract}${wasm_src_path}*.wasm ${wasm_dest_synthetic_token_path}
	cp ${wcspr_contract}${wasm_src_path}*.wasm ${wasm_dest_synthetic_token_path}
	cp ${flash_swapper_contract}${wasm_src_path}*.wasm ${wasm_dest_synthetic_token_path}
	cp ${pair_contract}${wasm_src_path}*.wasm ${wasm_dest_synthetic_token_path}
	cp ${library_contract}${wasm_src_path}*.wasm ${wasm_dest_synthetic_token_path}
	cp ${router_contract}${wasm_src_path}*.wasm ${wasm_dest_synthetic_token_path}
	cp ${liquidity_transformer_directory}/${wasm_src_path}*.wasm ${wasm_dest_synthetic_token_path}

build-contract:
    # Building transformer contracts
	cargo build --release -p liquidity_transformer -p synthetic_token -p scspr -p purse-proxy -p proxy_liquidity_transformer -p proxy_scspr --target wasm32-unknown-unknown

clean:
	cargo clean
	rm -rf liquidity_transformer_tests/wasm/*.wasm
	rm -rf scspr_tests/wasm/*.wasm
	rm -rf synthetic_token_tests/wasm/*.wasm
	rm -rf Cargo.lock

test-liquidity-transformer:
	cargo test -p liquidity_transformer_tests
test-scspr:
	cargo test -p scspr_tests
test-synthetic-token:
	cargo test -p synthetic_token_tests

test:
	make test-liquidity-transformer && make test-scspr && make test-synthetic-token

test-all:
	make all && make test

lint: clippy
	cargo fmt --all

check-lint: clippy
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all -- -D warnings

git-clean:
	git rm -rf --cached .
	git add .