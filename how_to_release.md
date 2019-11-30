# To create a release

The releases are automated via GitHub actions, using [this configuration file](https://github.com/Rigellute/spotify-tui/blob/master/.github/workflows/cd.yml).

The release is triggered by pushing a tag.

1. Bump the version in `Cargo.toml`
1. Update the "Unreleased" header for the new version in the `CHANGELOG`
1. Create a new tag e.g. `git tag -a v0.7.0`
1. Push the tag `git push --tags`
1. Wait for the build to finish on the [Actions page](https://github.com/Rigellute/spotify-tui/actions)
1. This should publish to cargo as well

### Update `brew`

1. Download the `sha` file, copy the `sha` into the tap repo formula and bump the version there too
1. Push to the tap repo with the new `sha` and version
