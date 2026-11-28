#!/usr/bin/env bash
set -euo pipefail

package_name="$1"
binary_name="$2"
build_target="$3"

version="$(python3 - <<'PY'
import tomllib

with open("Cargo.lock", "rb") as lockfile:
    packages = tomllib.load(lockfile)["package"]
versions = {package["version"] for package in packages if package["name"] == "wasm-bindgen"}
if len(versions) != 1:
    raise SystemExit("Expected exactly one wasm-bindgen version in Cargo.lock")
print(versions.pop())
PY
)"
cargo install --locked --version "$version" wasm-bindgen-cli
# Keep the module name imported by wasm/index.html.
wasm-bindgen \
  --out-name sudoku_machine \
  --out-dir wasm \
  --target web \
  "target/$build_target/release/$binary_name.wasm"
cp -r assets wasm/

cd wasm
zip --recurse-paths "../$package_name.zip" .
