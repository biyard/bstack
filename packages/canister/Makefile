CARGO_TARGET_DIR ?= $(PWD)/target

BUILD_ENV ?= CARGO_TARGET_DIR=$(CARGO_TARGET_DIR)

.PHONY: service.did
service.did:
	$(BUILD_ENV) cargo build --release --target wasm32-unknown-unknown
	candid-extractor $(CARGO_TARGET_DIR)/wasm32-unknown-unknown/release/canister.wasm > service.did

build: service.did
	$(BUILD_ENV) dfx build --playground
