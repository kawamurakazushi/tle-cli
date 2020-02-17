cargo build --release
cd target/release
tar -czf tle-0.1.0-x86_64-apple-darwin.tar.gz tle
shasum tle-0.1.0-x86_64-apple-darwin.tar.gz
