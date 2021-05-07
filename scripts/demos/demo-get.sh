#!/usr/bin/env bash

source env.sh

echo -e "The 'get' commnand allows fetching the runtime WASM from a live chain" | $human
sleep 0.5

echo -e "You can use either --url <...> or --chain <...>" | $human
sleep 0.5

echo -e "First let's download the latest Polkadot runtime" | $human
sleep 0.5

cmd="subwasm get --url wss://rpc.polkadot.io --output runtime.wasm"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd

sleep 0.5
echo -e "The latest Polkadot runtime is now available on our disk:" | $human
ls -al runtime.wasm
sleep 0.5

echo -e "Check the --help to see the other options, you can specify the name of the output file and also the block hash" | $human
subwasm get --help
sleep 1

echo -e
rm -f runtime.wasm
