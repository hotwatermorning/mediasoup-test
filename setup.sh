# #!/bin/bash

# set -e -u -o pipefail

# realpath() {
#   [[ $1 = /* ]] && echo "$1" || echo "$PWD/${1#./}"
# }

# SCRIPT_NAME="$(basename "$(realpath "${BASH_SOURCE:-$0}")")"
# SCRIPT_DIR="$(dirname "$(realpath "${BASH_SOURCE:-$0}")")"

# cd "${SCRIPT_DIR}"

# touch "./backend/.env.local"
# touch "./frontend/.env.local"
# touch "./backend/.env.local"
