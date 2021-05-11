#!/usr/bin/env bash

source env.sh

echo -e "subwasm can work on a runtime from your filsystem but can also directly work on the runtime from a live chain." | $human
echo -e "Let's get some information about the runtime on a live chain." | $human
cmd="subwasm info wss://rpc.polkadot.io"
echo -e "We use the following command:" | $human
echo -e "$cmd" | $human
$cmd
sleep 0.5

echo -e "Did you see that the proposal hash is also displayed here?" | $human
sleep 0.5
echo -e
