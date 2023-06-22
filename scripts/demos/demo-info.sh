#!/usr/bin/env bash

source env.sh

echo -e "subwasm can work on a runtime from your filesystem but can also directly work on the runtime from a live chain." | $human
echo -e "Let's get some information about the runtime on a live chain." | $human
cmd="subwasm -q info --chain polkadot"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd
sleep 0.5

echo -e "Do you see the proposal hash displayed here?" | $human
sleep 0.5
echo -e
