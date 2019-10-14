### To create a release

1. Bump the version in `Cargo.toml`
1. `sh ./build-release.sh <version>`
1. Copy the `sha` into the tap repo formula and bump the version there too
1. Upload the binary to github releases
1. Move the `CHANGELOG` docs to new release and copy/paste to the github release
1. Save the github release
1. Push to the tap repo with the new `sha` and version
1. `cargo publish --dry-run` to check everything is good
1. `cargo publish`
