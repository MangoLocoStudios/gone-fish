APP_NAME="GoneFish"
RUST_CRATE_NAME="gone-fish"
VERSION="1.0.1"
mkdir -p "${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_NAME}.app/Contents/Resources"
cp Info.plist "${APP_NAME}.app/Contents/Info.plist"
cp AppIcon.icns "${APP_NAME}.app/Contents/Resources/AppIcon.icns"
cp -a assets "${APP_NAME}.app/Contents/MacOS/"
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
lipo "target/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "target/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "${APP_NAME}.app/Contents/MacOS/${APP_NAME}"

# Create .dmg
mkdir -p "Builds/MacOS"
hdiutil create -fs HFS+ \
  -volname "Gone Fish" \
  -srcfolder "${APP_NAME}.app" \
  "Builds/MacOS/gone_fish_release_mac_${VERSION}.dmg"