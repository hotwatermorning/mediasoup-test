#!/bin/bash

set -e -u -o pipefail

# kill all background child processes when exit.
trap "exit" INT TERM
trap "kill 0" EXIT

realpath() {
  [[ $1 = /* ]] && echo "$1" || echo "$PWD/${1#./}"
}

SCRIPT_NAME="$(basename "$(realpath "${BASH_SOURCE:-$0}")")"
SCRIPT_DIR="$(dirname "$(realpath "${BASH_SOURCE:-$0}")")"

cd "${SCRIPT_DIR}"

# .trigger ファイルがないときのみ作成する
if [ ! -f .trigger ]; then
  touch .trigger
fi

cargo build
touch .env.local

cargo watch -x fmt -x build -s "touch .trigger" &
cargo watch --no-ignore -w .env.local -s "touch .trigger" &
cargo watch --no-ignore -w ".trigger" -x run
