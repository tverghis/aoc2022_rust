#!/bin/sh

if [ $# != 1 ]; then
	echo "Usage: $(basename "$0") <day>" >&2
	exit 1
fi

if [ ! -f "./.cookie" ]; then
	echo "No .cookie file found" >&2
	exit 1
fi

if [ ! -d .git ]; then
	echo "Must be run from the repository root" >&2
	exit 1
fi

name="$(printf "day%02d" "$1")"
cookie="$(cat ./.cookie)"
echo "Creating directory for $name..."
cargo new --bin "$name"
echo "Downloading the input for Day $1..."
curl -H "Cookie: $cookie" https://adventofcode.com/2022/day/"$1"/input > "$name"/input.txt
echo "Setup complete!"
