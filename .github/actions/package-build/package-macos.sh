#!/usr/bin/env bash
set -eo pipefail

app_name="$1"
package_name="$2"
binary_name="$3"
build_target="$4"
artifact_name="$5"
version="$6"

bundle="$app_name.app"
mkdir -p "$bundle/Contents/MacOS"
cp "target/$build_target/release/$binary_name" "$bundle/Contents/MacOS/"
cp -r assets "$bundle/Contents/MacOS/"

minimum_os="$(rustc --print=deployment-target --target "$build_target")"
cat > "$bundle/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleDisplayName</key>
    <string>$app_name</string>
    <key>CFBundleExecutable</key>
    <string>$binary_name</string>
    <key>CFBundleIdentifier</key>
    <string>dev.$GITHUB_REPOSITORY_OWNER.$package_name</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$app_name</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$version</string>
    <key>CFBundleVersion</key>
    <string>$version</string>
    <key>LSMinimumSystemVersion</key>
    <string>${minimum_os#*=}</string>
</dict>
</plist>
EOF
plutil -lint "$bundle/Contents/Info.plist"

signing_args=()
if [[ -n "${SIGNING_IDENTITY:-}" ]]; then
  codesign --force --deep --sign "$SIGNING_IDENTITY" "$bundle"
  signing_args=(--codesign "$SIGNING_IDENTITY")
fi

mkdir dmg-content
mv "$bundle" dmg-content/
create-dmg \
  --volname "$package_name" \
  --background assets/images/dmg-background.png \
  --window-size 512 210 \
  --icon-size 128 \
  --icon "$bundle" 128 64 \
  --hide-extension "$bundle" \
  --app-drop-link 384 64 \
  "${signing_args[@]}" \
  "$package_name-$artifact_name.dmg" \
  dmg-content
