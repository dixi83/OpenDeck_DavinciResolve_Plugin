# Releases

Releases are created by pushing a version tag. The GitHub Actions workflow in
`.github/workflows/release.yml` builds the Windows OpenDeck plugin and attaches
`DaVinciResolve-OpenDeck.sdPlugin.zip` to the GitHub Release for that tag.

## First release: 0.1.0

The first release uses tag `v0.1.0` and matches the plugin version in
`assets/manifest.json`.

To publish a later release:

1. Update the version in `assets/manifest.json` and any release notes.
2. Commit the changes to `main`.
3. Create and push a matching tag, for example:

   ```powershell
   git tag v0.1.1
   git push origin main
   git push origin v0.1.1
   ```

4. Check the **Actions** tab for the workflow, then review the generated GitHub
   Release and attached ZIP.

Do not commit the generated ZIP, plugin package directory, or Cargo build output;
the workflow builds those for each release.
