# Anoma ledger prototype

## Quick start

The ledger currently requires that [Tendermint version 0.34.x](https://github.com/tendermint/tendermint) is installed and available on path. [The pre-built binaries and the source for 0.34.8 are here](https://github.com/tendermint/tendermint/releases/tag/v0.34.8), also directly available in some package managers.

The transaction code can currently be built from [tx_template](txs/tx_template) and validity predicates from [vp_template](vps/vp_template), which is Rust code compiled to wasm.

The transaction template calls functions from the host environment. The validity predicate template can validate a transaction and the key changes that is has performed.

The validity predicate is currently hard-coded in the shell and used for every account. To experiment with a different validity predicate, build it from the template and restart the shell.

The gossip node needs to toggle the intent flag `--intent` to activate the intent broadcaster, multiple nodes can be connected with the `--peers` option.

The matchmaker template receives intents with the borsh encoding define in `data_template` and crafts data to be sent with `tx_intent_template` to the ledger. It matches only two intents that are the exact opposite.

```shell
# Install development dependencies
make dev-deps

# Run this first if you don't have Rust wasm target installed:
make -C txs/tx_template deps

# Build the validity predicate, transaction and matchmaker wasm modules:
make build-wasm-scripts

# Build Anoma
make

# Build and link the executables
make install

# generate default config in .anoma/
cargo run --bin anoman -- --base-dir .anoma generate-config

# Run Anoma node (this will also initialize and run Tendermint node)
make run-ledger

# Reset the state (resets Tendermint too)
cargo run --bin anoman -- reset-ledger

# Submit a custom transaction with a wasm code and arbitrary data in `tx.data` file.
# Note that you have to have a `tx.data` file for this to work, albeit it can be empty.
cargo run --bin anoma -- tx --code-path txs/tx_template/tx.wasm --data-path tx.data

# User addresses
export adrian=a1qq5qqqqqxgeyzdeng4zrw33sxez5y3p3xqerz3psx5e5x32rg3rryw2ygc6yy3p4xpq5gvfnw3nwp8
export alberto=a1qq5qqqqq8yerw3jxx565y333gfpnjwzygcc5zd6xxarr2dzzgcm5xv3kxazrjve589p5vv34vl0yy3
export ash=a1qq5qqqqqxue5vs69xc6nwvfcgdpyy3pnxv6rxw2zx3zryv33gyc5xdekxaryydehgvunsvzz2hjedu
export awa=a1qq5qqqqqg565zv34gcc52v3nxumr23z9gezrj3pnx56rwse4xc6yg3phgcun2d33xyenqv2x4xyw62
export celso=a1qq5qqqqq8qmrwsjyxcerqwzpx9pnzve3gvc5xw29gdqnvv2yx5mrvsjpxgcrxv6pg5engvf5hgjscj
export chris=a1qq5qqqqqgye5xwpcxqu5z3p4g5ens3zr8qm5xv69xfznvwzzx4p5xwpkxc6n2v6x8yc5gdpeezdqc4
export gabriella=a1qq5qqqqq8ycn2djrxqmnyd3sxcunsv2zgyeyvwzpgceyxdf3xyu5gv2pgeprxdfe8ycrzwzzkezpcp
export gianmarco=a1qq5qqqqq89prqsf38qcrzd6zxym5xdfjg4pyg3pjg3pyx32zg5u5y3jpgc65zdej8pznwwf3jqzsws
export joe=a1qq5qqqqqgvuyv335g9z5v32xgdz523zxgsuy23fjxazrjve5g4pnydphxyu5v33cxarrzd692045xh
export nat=a1qq5qqqqq89rygsejx9q5yd6pxpp5x3f38ymyydp3xcu523zzx4prw3fc8qu5vvjpxyeyydpnfha6qt
export simon=a1qq5qqqqqgfqnqdecxcurq33hxcey2sf4g5mygdjyxfrrjse4xyc52vpjxyenwve4gv6njsecz4tzen
export sylvain=a1qq5qqqqqgccnyvp3gyergvp5xgmr2s3s8yung3f4gdq52wzpxvurysfhgycnwd29xfryxvekfwc00t
export tomas=a1qq5qqqqqggcrzsfj8ym5g3psxuurxv2yxseyxwpsxdpy2s35gsc5zdzpx9pyxde48ppnqd3cnzlava
export yuji=a1qq5qqqqqgvcrz3f5x4prssj9x5enydecxfznzdj9g5cnj3fcxarrxdjpx5cnwv69xye5vvfeva4z85

# Token Addresses
export XAN=a1qq5qqqqqxuc5gvz9gycryv3sgye5v3j9gvurjv34g9prsd6x8qu5xs2ygdzrzsf38q6rss33xf42f3
export BTC=a1qq5qqqqq8q6yy3p4xyurys3n8qerz3zxxeryyv6rg4pnxdf3x3pyv32rx3zrgwzpxu6ny32r3laduc
export ETH=a1qq5qqqqqxeryzvjxxsmrj3jpxapygve58qerwsfjxaznvd3n8qenyv2ygsc52335xue5vve5m66gfm
export XTZ=a1qq5qqqqqx3z5xd3ngdqnzwzrgfpnxd3hgsuyx3phgfry2s3kxsc5xves8qe5x33sgdprzvjptzfry9
export DOGE=a1qq5qqqqqx9rrq3zrg5myzv3eg9zyxvf3gery2dfhgg6nsdjrxscrgv6rgsunx33sxg6nvdjrkujezz

# Submit a token transfer
cargo run --bin anomac -- transfer --source $ALAN --target $ADA --token $XAN --amount 10.1 --code-path txs/tx_transfer/tx.wasm

# Submit a transaction to update an account's validity predicate
cargo run --bin anomac -- update --address $ALAN --code-path vps/vp_user/vp.wasm

# Watch and on change run a node (the state will be persisted)
cargo watch -x "run --bin anoman -- run-ledger"

# Watch and on change reset & run a node
cargo watch -x "run --bin anoman -- reset-ledger" -x "run --bin anoman -- run"

# run gossip node with intent broadcaster and rpc server (use default config)
cargo run --bin anoma -- run-gossip --rpc

# run gossip node with intent broadcaster, matchmaker and rpc (use default config)
cargo run --bin anoman -- run-gossip --rpc --matchmaker-path matchmaker_template/matchmaker.wasm --tx-code-path txs/tx_from_intent/tx.wasm --ledger-address "127.0.0.1:26657"

# craft intents
cargo run --bin anomac -- craft-intent --address $ADA    --token-buy $XTZ --amount-buy 10 --token-sell $BTC --amount-sell 20 --file-path intent_A.data
cargo run --bin anomac -- craft-intent --address $ALAN   --token-buy $BTC --amount-buy 20 --token-sell $XAN --amount-sell 30 --file-path intent_B.data
cargo run --bin anomac -- craft-intent --address $ALONZO --token-buy $XAN --amount-buy 30 --token-sell $XTZ --amount-sell 10 --file-path intent_C.data

# Subscribe to new network
cargo run --bin anomac -- subscribe-topic --node "http://[::1]:39111" --topic "asset_v1"

# Submit the intents (need a rpc server), hardcoded address rpc node address
cargo run --bin anomac -- intent --node "http://[::1]:39111" --data-path intent_A.data --topic "asset_v1"
cargo run --bin anomac -- intent --node "http://[::1]:39111" --data-path intent_B.data --topic "asset_v1"
cargo run --bin anomac -- intent --node "http://[::1]:39111" --data-path intent_C.data --topic "asset_v1"

# Format the code
make fmt
```

## Logging

To change the log level, set `ANOMA_LOG` environment variable to one of:
- `error`
- `warn`
- `info`
- `debug`
- `trace`

The default is set to `info` for all the modules, expect for Tendermint ABCI, which has a lot of `debug` logging.

For more fine-grained logging levels settings, please refer to the [tracing subscriber docs](https://docs.rs/tracing-subscriber/0.2.18/tracing_subscriber/struct.EnvFilter.html#directives) for more information.
