#!/bin/bash

set -e

if [[ -z "$1" ]]; then
  >&2 echo "Tell me which day to fetch to"
  exit 1
fi

for n in `seq 1 $1`; do
  if [[ -s ./in/$n.txt ]]; then
    echo "skipping $n.txt because it already exists"
  else
    curl -s -f -o ./in/$n.txt -b cookie.jar https://adventofcode.com/2025/day/$n/input
    echo "done $n.txt"
  fi
done

