#!/usr/bin/env bash
set -euo pipefail

package_name="$1"
binary_name="$2"
build_target="$3"

mkdir linux
cp "target/$build_target/release/$binary_name" linux/
cp -r assets linux/

cd linux
zip --recurse-paths "../$package_name.zip" .
