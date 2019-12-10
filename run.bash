#!/bin/bash
for input in inputs/*; do
    day=$(basename $input | cut -f 1 -d '.')
    cat inputs/$day.txt | cargo run -p day_$day --release
done
