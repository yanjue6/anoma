package = anoma
version = $(shell git describe --dirty --broken)
platform = $(shell uname -s)-$(shell uname -m)
package-name = anoma-$(version)-$(platform)

bin = anoma anomac anoman anomaw

cargo := $(env) cargo
rustup := $(env) rustup
debug-env := RUST_BACKTRACE=1 RUST_LOG=$(package)=debug
debug-cargo := $(env) $(debug-env) cargo
# Nightly build is currently used for rustfmt and clippy.
nightly := $(shell cat rust-nightly-version)

ifdef IN_NIX_SHELL
cargo-clippy := cargo-clippy
cargo-fmt := cargo-fmt
cargo-miri := cargo-miri
else
cargo-clippy := $(cargo) +$(nightly) clippy
cargo-fmt := $(cargo) +$(nightly) fmt
cargo-miri := $(cargo) +$(nightly) miri
endif

# Path to the wasm source for the provided txs and VPs
wasms := wasm/wasm_source
# Paths for all the wasm templates
wasm_templates := wasm/tx_template wasm/vp_template wasm/mm_template wasm/mm_filter_template

# Transitive dependency warning from tendermint-rpc
audit-ignores += RUSTSEC-2020-0016
# Transitive dependency warning from tendermint-rs and ibc-rs
# TODO https://github.com/anoma/anoma/issues/340
audit-ignores += RUSTSEC-2021-0073
# TODO upgrade libp2p
audit-ignores += RUSTSEC-2021-0076

build:
	$(cargo) build

build-test:
	$(cargo) build --tests

build-release:
	ANOMA_DEV=false $(cargo) build --release --package anoma_apps

check-release:
	ANOMA_DEV=false $(cargo) check --release --package anoma_apps

package: build-release
	mkdir -p $(package-name)/wasm && \
	cd target/release && ln $(bin) ../../$(package-name) && \
	cd ../.. && \
	ln wasm/checksums.json $(package-name)/wasm && \
	tar -c -z -f $(package-name).tar.gz $(package-name) && \
	rm -rf $(package-name)

build-release-image-docker:
	docker build -t anoma-build .

build-release-docker: build-release-image-docker
	docker run --rm -v ${PWD}:/var/build anoma-build make build-release

package-docker: build-release-image-docker
	docker run --rm -v ${PWD}:/var/build anoma-build make package

check-wasm = $(cargo) check --target wasm32-unknown-unknown --manifest-path $(wasm)/Cargo.toml
check:
	$(cargo) check && \
	make -C $(wasms) check && \
	$(foreach wasm,$(wasm_templates),$(check-wasm) && ) true

clippy-wasm = $(cargo-clippy) --manifest-path $(wasm)/Cargo.toml --all-targets -- -D warnings
clippy:
	$(cargo-clippy) --all-targets -- -D warnings && \
	make -C $(wasms) clippy && \
	$(foreach wasm,$(wasm_templates),$(clippy-wasm) && ) true

clippy-fix:
	$(cargo-clippy) --fix -Z unstable-options --all-targets --allow-dirty --allow-staged

install: tendermint
	ANOMA_DEV=false $(cargo) install --path ./apps

tendermint:
	./scripts/install/get_tendermint.sh

run-ledger:
	# runs the node
	$(cargo) run --bin anoman -- ledger run

run-gossip:
	# runs the node gossip node
	$(cargo) run --bin anoman -- gossip run

reset-ledger:
	# runs the node
	$(cargo) run --bin anoman -- ledger reset

audit:
	$(cargo) audit $(foreach ignore,$(audit-ignores), --ignore $(ignore))

test: test-unit test-e2e test-wasm

test-e2e:
	RUST_BACKTRACE=1 $(cargo) test e2e -- --test-threads=1

test-unit:
	$(cargo) test -- --skip e2e

test-wasm:
	make -C $(wasms) test

test-wasm-template = $(cargo) test --manifest-path $(wasm)/Cargo.toml
test-wasm-templates:
	$(foreach wasm,$(wasm_templates),$(test-wasm-template) && ) true

test-debug:
	$(debug-cargo) test -- --nocapture

fmt-wasm = $(cargo-fmt) --manifest-path $(wasm)/Cargo.toml
fmt:
	$(cargo-fmt) --all && \
	make -C $(wasms) fmt && \
	$(foreach wasm,$(wasm_templates),$(fmt-wasm) && ) true

fmt-check-wasm = $(cargo-fmt) --manifest-path $(wasm)/Cargo.toml -- --check
fmt-check:
	$(cargo-fmt) --all -- --check && \
	make -C $(wasms) fmt-check && \
	$(foreach wasm,$(wasm_templates),$(fmt-check-wasm) && ) true

watch:
	$(cargo) watch

clean:
	$(cargo) clean

build-doc:
	$(cargo) doc --no-deps
	make -C docs build

doc:
	# build and opens the docs in browser
	$(cargo) doc --open

build-wasm-image-docker:
	docker build -t anoma-wasm wasm

build-wasm-scripts-docker: build-wasm-image-docker
	docker run --rm -v ${PWD}:/usr/local/rust/wasm anoma-wasm make build-wasm-scripts

# Build the validity predicate, transactions, matchmaker and matchmaker filter wasm
build-wasm-scripts:
	make -C $(wasms)
	make opt-wasm
	make checksum-wasm

# need python
checksum-wasm:
	python wasm/checksums.py

# this command needs wasm-opt installed
opt-wasm:
	@for file in $(shell ls wasm/*.wasm); do wasm-opt -Oz -o $${file} $${file}; done

clean-wasm-scripts:
	make -C $(wasms) clean

dev-deps:
	$(rustup) toolchain install $(nightly)
	$(rustup) target add wasm32-unknown-unknown
	$(rustup) component add rustfmt clippy miri --toolchain $(nightly)
	$(cargo) install cargo-watch

test-miri:
	$(cargo-miri) setup
	$(cargo) clean
	MIRIFLAGS="-Zmiri-disable-isolation" $(cargo-miri) miri test

.PHONY : build check build-release clippy install run-ledger run-gossip reset-ledger test test-debug fmt watch clean build-doc doc build-wasm-scripts-docker build-wasm-scripts clean-wasm-scripts dev-deps test-miri
