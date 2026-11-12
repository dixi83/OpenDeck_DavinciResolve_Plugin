# Releases

Releases are created by pushing a version tag. The GitHub Actions workflow in
`.github/workflows/release.yml` builds the Windows OpenDeck plugin and attaches
`DaVinciResolve-OpenDeck.sdPlugin.zip` to the GitHub Release for that tag.

## First release: 0.1.0

The first release was `v0.1.0`. Version `v0.1.1` updates the project's license
to GPLv3-only and matches the plugin version in `assets/manifest.json`.

To publish a later release:

1. Update the versions in `assets/manifest.json` and `src/Cargo.toml`, and add
   release notes.
2. Commit the changes to `main`.
3. Create and push a matching tag, for example:

   ```powershell
   git tag v0.1.2
   git push origin main
   git push origin v0.1.2
   ```

4. Check the **Actions** tab for the workflow, then review the generated GitHub
   Release and attached ZIP.

Do not commit the generated ZIP, plugin package directory, or Cargo build output;
the workflow builds those for each release.
