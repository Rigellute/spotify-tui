#!/usr/bin/env bash

# Would be nice to do this on CI rather than manually
version=$1
tar_name="spotify-tui-v${version}.tar.gz"

echo "Building $version"

cargo build --release

cd target/release
tar -czf $tar_name spt

shasum -a 256 $tar_name


