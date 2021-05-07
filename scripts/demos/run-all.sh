#!/usr/bin/env bash
. env.sh

mkdir -p casts svg

demos=( \
    demo-get \
    demo-info \
    demo-meta \
    demo-diff \
)

for demo in ${demos[@]}; do
    echo Recoding demos:
    echo - $demo
    asciinema rec --overwrite casts/$demo.cast -c ./$demo.sh
    termtosvg render -t window_frame_js casts/$demo.cast svg/$demo.svg -D 3000
done

rm -f *.wasm
