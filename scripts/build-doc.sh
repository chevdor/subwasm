#!/usr/bin/env bash

cargo doc \
  --workspace \
  -F default --no-deps

echo "<meta http-equiv=\"refresh\" content=\"0; url=subwasm\">" > target/doc/index.html
