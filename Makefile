deploy.canister:
	cd packages/canister && make build
	dfx deploy --playground
