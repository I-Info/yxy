#! /bin/sh
echo "Building libraries..."
cargo build -r --target aarch64-apple-darwin 
cargo build -r --target aarch64-apple-ios
cargo build -r --target aarch64-apple-ios-sim
cargo build -r --target x86_64-apple-darwin
cargo build -r --target x86_64-apple-ios

echo "Creating fat libraries..."
rm -rf target/universal/release

mkdir -p target/universal/release

lipo -create \
		 target/x86_64-apple-darwin/release/libyxy.a \
		 target/aarch64-apple-darwin/release/libyxy.a \
	 -output target/universal/release/libyxy_macos.a

lipo -create \
		 target/aarch64-apple-ios-sim/release/libyxy.a \
		 target/x86_64-apple-ios/release/libyxy.a \
	 -output target/universal/release/libyxy_iossim.a



echo "Creating XCFramework bundle..."
rm -rf target/yxy.xcframework

xcodebuild -create-xcframework \
	-library ./target/universal/release/libyxy_macos.a -headers ./include \
	-library ./target/universal/release/libyxy_iossim.a -headers ./include \
	-library ./target/aarch64-apple-ios/release/libyxy.a -headers ./include \
	-output target/yxy.xcframework

