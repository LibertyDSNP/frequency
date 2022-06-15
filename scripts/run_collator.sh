#!/usr/bin/env bash

# this script runs the mrc after fetching
# appropriate bootnode IDs

set -e -o pipefail

ctpc="${MRC_BINARY_PATH:-./target/release/mrc-collator}"

if [ ! -x "$ctpc" ]; then
    echo "FATAL: $ctpc does not exist or is not executable"
    exit 1
fi

# name the variable with the incoming args so it isn't overwritten later by function calls
args=( "$@" )

alice="${HOST_ALICE:-relay_alice}"
bob="${HOST_BOB:-relay_bob}"
alice_rpc_port="${ALICE_RPC_PORT:-9934}"
bob_rpc_port="${BOB_RPC_PORT:-9935}"
chain="${RELAY_CHAIN_SPEC:-./res/rococo-local.json}"

get_bootnode () {
    node="$1"
    port="$2"
    SELECT_INDEX=`[[ "$alice" == "127.0.0.1" ]] && echo "0" || echo "1"`
    curl -sS \
        -H 'Content-Type: application/json' \
        --data '{"id":1,"jsonrpc":"2.0","method":"system_localListenAddresses"}' \
        "$node:$port" |\
    tee |\
    jq -r '.result['$SELECT_INDEX'] // ""'

}

bootnode () {
    node="$1"
    rpc_port="$2"
    bootnode=$(get_bootnode "$node" "$rpc_port")
    if [ -z "$bootnode" ]; then
        echo >&2 "failed to get id for $node"
        # curl -vsS \
        # -H 'Content-Type: application/json' \
        # --data '{"id":1,"jsonrpc":"2.0","method":"localListenAddresses"}' \
        # "$node:$rpc_port"
        exit 1
    fi
    >&2 echo "Bootnode: $bootnode"
    echo "$bootnode"
}

args+=( "--" "--wasm-execution=compiled" "--execution=wasm" "--chain=${chain}" "--bootnodes=$(bootnode "$alice" "$alice_rpc_port")" "--bootnodes=$(bootnode "$bob" "$bob_rpc_port")" )

set -x
"$ctpc" "${args[@]}"
