#!/usr/bin/env sh

cd "$(dirname "$0")"

for i in $(seq 25); do
    sed "s/{{N}}/$i/g" skel.rs > "day$i.rs"
    echo "pub mod day$i;"
done
