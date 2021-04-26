#!/usr/bin/env bash

cargo run --release \
    | jq '.[1]' \
    | jq 'map(del(.. | .documentation?))' \
    | jq 'map(del(.. | .default?))' \
    | jq 'map(del(.. | .value?))' \
    | jq 'map(del(.. | .calls?))' \
    | jq 'map(del(.. | .event?))' \
    | jq .[0]> cleaned.json
