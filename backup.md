fs_extra = { version = "1.3.0" }
kitchensink-runtime = { path = "substrate/bin/node/runtime", version = "3.0.0" }
node-testing = { path = "substrate/bin/node/testing", version = "3.0.0" }
node-cli = { path = "substrate/bin/node/cli", package = "staging-node-cli", version = "3.0.0" }
pallet-asset-conversion = { path = "substrate/frame/asset-conversion", default-features = false, version = "21.1.0" }
pallet-asset-conversion-tx-payment = { path = "substrate/frame/transaction-payment/asset-conversion-tx-payment", default-features = false, version = "21.1.0" }
pallet-asset-tx-payment = { path = "substrate/frame/transaction-payment/asset-tx-payment", default-features = false, version = "39.1.0" }
pallet-skip-feeless-payment = { path = "substrate/frame/transaction-payment/skip-feeless-payment", default-features = false, version = "14.1.0" }
node-inspect = { path = "substrate/bin/node/inspect", default-features = false, package = "staging-node-inspect", version = "0.26.0" }
node-rpc = { path = "substrate/bin/node/rpc", version = "3.0.0" }
subxt-signer = { version = "0.37" }
assert_cmd = { version = "2.0.14" }
nix = { version = "0.28.0" }
platforms = { version = "3.4" }
soketto = { version = "0.8.0" }
wait-timeout = { version = "0.2" }
clap_complete = { version = "4.5.13" }
mmr-rpc = { path = "substrate/client/merkle-mountain-range/rpc", default-features = false, version = "39.0.0" }
sc-consensus-babe-rpc = { path = "substrate/client/consensus/babe/rpc", default-features = false, version = "0.48.0" }
sc-consensus-beefy = { path = "substrate/client/consensus/beefy", default-features = false, version = "27.0.0" }
sc-consensus-beefy-rpc = { path = "substrate/client/consensus/beefy/rpc", default-features = false, version = "27.0.0" }
sc-consensus-grandpa-rpc = { path = "substrate/client/consensus/grandpa/rpc", default-features = false, version = "0.33.0" }
sc-sync-state-rpc = { path = "substrate/client/sync-state-rpc", default-features = false, version = "0.48.0" }
sp-consensus-beefy = { path = "substrate/primitives/consensus/beefy", default-features = false, version = "23.0.0" }
substrate-state-trie-migration-rpc = { path = "substrate/utils/frame/rpc/state-trie-migration-rpc", default-features = false, version = "41.0.0" }
num-rational = { version = "0.4.1" }
sc-consensus-epochs = { path = "substrate/client/consensus/epochs", default-features = false, version = "0.47.0" }
substrate-rpc-client = { path = "substrate/utils/frame/rpc/client", default-features = false, version = "0.47.0" }
cmd_lib = { version = "1.9.5" }
sp-authority-discovery = { path = "substrate/primitives/authority-discovery", default-features = false, version = "35.0.0" }
sp-consensus-pow = { path = "substrate/primitives/consensus/pow", default-features = false, version = "0.41.0" }
fs4 = { version = "0.7.0" }
pallet-default-config-example = { path = "substrate/frame/examples/default-config", default-features = false, version = "10.0.0" }
pallet-dev-mode = { path = "substrate/frame/examples/dev-mode", default-features = false, version = "21.1.0" }
pallet-example-authorization-tx-extension = { path = "substrate/frame/examples/authorization-tx-extension", default-features = false, version = "1.0.0" }
pallet-example-basic = { path = "substrate/frame/examples/basic", default-features = false, version = "34.0.0" }
pallet-example-frame-crate = { path = "substrate/frame/examples/frame-crate", default-features = false, version = "0.1.0" }
pallet-example-kitchensink = { path = "substrate/frame/examples/kitchensink", default-features = false, version = "4.0.0" }
pallet-example-offchain-worker = { path = "substrate/frame/examples/offchain-worker", default-features = false, version = "28.0.0" }
pallet-example-single-block-migrations = { path = "substrate/frame/examples/single-block-migrations", default-features = false, version = "0.1.0" }
pallet-example-split = { path = "substrate/frame/examples/split", default-features = false, version = "10.0.0" }
pallet-examples = { path = "substrate/frame/examples", version = "7.0.0" }
frame = { path = "substrate/frame", default-features = false, package = "polkadot-sdk-frame", version = "0.8.1" }
lite-json = { version = "0.2.0", default-features = false }
pallet-collective = { path = "substrate/frame/collective", default-features = false, version = "39.1.0" }
pallet-identity = { path = "substrate/frame/identity", default-features = false, version = "39.1.0" }
enumflags2 = { version = "0.7.7" }
indicatif = { version = "0.17.7" }
spinners = { version = "4.1.1" }
tokio-retry = { version = "0.3.0" }
pallet-beefy = { path = "substrate/frame/beefy", default-features = false, version = "40.1.0" }
pallet-mmr = { path = "substrate/frame/merkle-mountain-range", default-features = false, version = "39.0.0" }
pallet-treasury = { path = "substrate/frame/treasury", default-features = false, version = "38.1.0" }
bitvec = { version = "1.0.1", default-features = false }
pallet-bounties = { path = "substrate/frame/bounties", default-features = false, version = "38.1.0" }
xcm = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2412-4", default-features = false, package = "staging-xcm", version = "15.0.2" }
xcm-builder = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2412-4", default-features = false, package = "staging-xcm-builder", version = "18.1.0" }
pallet-xcm = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2412-4", default-features = false, version = "18.1.0" }
polkadot-parachain-primitives = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2412-4", default-features = false, version = "15.0.0" }
polkadot-primitives = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2412-4", default-features = false, version = "17.1.0" }
polkadot-runtime-parachains = {git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2412-4", default-features = false, version = "18.1.0" }
xcm-executor = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2412-4", default-features = false, package = "staging-xcm-executor", version = "18.0.2" }
xcm-simulator = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2412-4", default-features = false, version = "18.0.0" }
pallet-ranked-collective = { path = "substrate/frame/ranked-collective", default-features = false, version = "39.0.0" }
