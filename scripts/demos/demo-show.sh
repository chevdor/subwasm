#!/usr/bin/env bash

source env.sh

echo -e "subwasm knows cool tricks. It can show you the metadata directly from the WASM without connecting to any node." | $human
echo -e "You may generate the WASM using srtool for instance, here we keep it simple and fetch it from a node with:" | $human

cmd="subwasm -q get --chain polkadot --output runtime.wasm"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd
sleep 0.5

echo -e "Let's ask subwasm to call into the runtime and show us the list of pallets and their content:" | $human
cmd="subwasm -q show runtime.wasm"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd
sleep 0.5

echo -e
rm -f runtime.wasm
