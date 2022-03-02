#!/bin/bash
set -e

cargo +stable build --target wasm32-unknown-unknown --release

RELEASEDIR="target/wasm32-unknown-unknown/release"

cp $RELEASEDIR/ambassadors_dao.wasm ./ambassadors-dao/res/
cp $RELEASEDIR/ambassadors_dao_factory.wasm ./ambassadors-dao-factory/res/

# TODO: create a github workflow to run tests and cargo clippy
