#!/usr/bin/env bash
set -euo pipefail

package_name="$1"
binary_name="$2"
build_target="$3"

cargo install --locked --version "=${WASM_BINDGEN_VERSION:?Expected the locked wasm-bindgen version}" wasm-bindgen-cli
# Keep the module name imported by wasm/index.html.
wasm-bindgen \
  --out-name sudoku_machine \
  --out-dir wasm \
  --target web \
  "target/$build_target/release/$binary_name.wasm"
cp -r assets wasm/

cd wasm
zip --recurse-paths "../$package_name.zip" .
