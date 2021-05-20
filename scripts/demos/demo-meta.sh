#!/usr/bin/env bash

source env.sh

echo -e "subwasm knows cool tricks. It can show you the metadata directly from the WASM without connecting to any node." | $human
echo -e "You may generate the WASM using srtool for instance, here we keep it simple and fetch it from a node with:" | $human

cmd="subwasm get wss://rpc.polkadot.io --output runtime.wasm"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd
sleep 0.5

echo -e "Let's ask subwasm to call into the runtime and show us the list of modules from the metadata:" | $human
cmd="subwasm meta runtime.wasm"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd
sleep 0.5

echo -e "Do you prefer the output as json ?:" | $human
cmd="subwasm meta --json runtime.wasm | tail"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd
sleep 0.5

echo -e "We can also zoom in and see some information about a specific module:" | $human
cmd="subwasm meta runtime.wasm --module system"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd
sleep 0.5

echo -e
rm -f runtime.wasm
