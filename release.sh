# Building binaries
cargo build --release --target=x86_64-unknown-linux-gnu
cross build --release --target=x86_64-apple-darwin
cross build --release --target=arm-apple-darwin
cross build --release --target=armv7-unknown-linux-gnueabihf
cross build --release --target=x86_64-pc-windows-gnu

# Moving binaries
mkdir -p dist/darwin_amd64 && cp target/x86_64-apple-darwin/release/resin dist/darwin_amd64
mkdir -p dist/darwin_arm64 && cp target/arm-apple-darwin/release/resin dist/darwin_arm64
mkdir -p dist/linux_amd64 && cp target/x86_64-unknown-linux-gnu/release/resin dist/linux_amd64
mkdir -p dist/linux_armv7 && cp target/armv7-unknown-linux-gnueabihf/release/resin dist/linux_armv7
mkdir -p dist/windows_amd64 && cp target/x86_64-pc-windows-gnu/release/resin dist/windows_amd64
