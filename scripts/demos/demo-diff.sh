#!/usr/bin/env bash

source env.sh

echo -e "Would that not be cool to be able to diff 2 runtimes ? This is precisely what the diff command does!" | $human

cmd="subwasm -q get --chain polkadot --output runtime_latest.wasm"
echo -e "$cmd" | $human
$cmd
echo -e

cmd="subwasm -q diff runtime_latest.wasm runtime_latest.wasm"
echo -e "Let's check identity first!" | $human
echo -e "$cmd" | $human
$cmd
sleep 3
echo -e

echo -e
rm -f "*.wasm"
