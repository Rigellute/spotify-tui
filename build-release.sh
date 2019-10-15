#!/usr/bin/env bash

# Would be nice to do this on CI rather than manually
version=$1
tar_name="spotify-tui-v${version}.tar.gz"
release_path="target/release/"

echo "Building $version"

cargo build --release

cd $release_path
tar -czf $tar_name spt

shasum -a 256 $tar_name


