#!/usr/bin/env sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
VERUS="${VERUS:-$SCRIPT_DIR/verus}"
EXPECTED_SHA256=562588748e3587043fa23361448b934ad88b32f7be3d6457938a7a417ec775f8

test -x "$VERUS"
"$VERUS" --version | grep -F 'Version: 0.2025.03.28.892067f' >/dev/null
test "$(sha256sum "$VERUS" | awk '{print $1}')" = "$EXPECTED_SHA256"
exec "$VERUS" "$SCRIPT_DIR/proof.rs"
