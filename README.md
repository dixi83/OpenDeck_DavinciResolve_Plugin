# DaVinci Resolve controls for OpenDeck

An OpenDeck plugin with one-tap Edit page controls. The first release targets Windows and sends Resolve's default keyboard shortcuts to the active application.

## Actions

| Action | Default shortcut |
| --- | --- |
| Edit page | Shift+4 |
| Play / pause | Space |
| Reverse / stop / forward | J / K / L |
| Previous / next frame | Left / Right |
| Previous / next edit | Up / Down |
| Mark in / mark out | I / O |
| Add marker | M |
| Insert Clip | F9 |
| Split at playhead | Ctrl+B |
| Undo / redo | Ctrl+Z / Ctrl+Shift+Z |
| Selection / blade tool | A / B |

The **Timeline Jog** encoder action focuses the Edit page timeline and moves its playhead (clockwise forward, counterclockwise backward). **Source Clip Jog** focuses the Source Viewer and jogs the loaded source clip. Both accelerate from 1 to 2, 4, or 8 frames per detent as rotation speeds up. Pressing either encoder toggles play/pause. These two actions are intended for the Edit page and use Resolve's default Windows focus shortcuts (Ctrl+4 for Timeline, Ctrl+3 for Source Viewer).

The **Timeline Zoom** encoder focuses the Edit page timeline, then uses Ctrl+= / Ctrl+- to zoom in or out. Pressing the encoder uses Shift+Z to fit the whole timeline in view. Rotation is capped at 12 zoom steps per input event.

## Page navigation

Media, Photo, Cut, Edit, Fusion, Color, Fairlight, and Deliver each have a page action. They use Resolve's bundled Python scripting API so Photo is supported even though it has no default page shortcut. DaVinci Resolve must be running, with local external scripting enabled in Preferences.

Resolve's keyboard shortcuts can be customized. These actions use the defaults shown above and send input to whichever application is focused. For predictable results, focus DaVinci Resolve and the timeline or viewer before pressing a key. The Split action follows Resolve's current selection and track targeting rules.

## Build

Requires Rust and Cargo. From `src`:

```powershell
cargo build --release --target x86_64-pc-windows-msvc
```

Run `./build.ps1` from PowerShell to produce `DaVinciResolve-OpenDeck.sdPlugin.zip`. Install that archive from OpenDeck's plugin manager.

## License and release

The plugin source and original DiXY icons are licensed under the GNU General Public License v3.0 only; see [LICENSE](LICENSE). Commercial use is allowed, including using the plugin to edit videos for paying clients or to create work that you sell. GPLv3 also permits recipients to redistribute the covered source and binaries under its terms.

The third-party icon pack and its copied action icons are not covered by the project license; they retain their upstream MIT terms. See [third-party notices](THIRD-PARTY-NOTICES.md) and [the icon pack's MIT license](assets/icons/iconpack/LICENSE-MIT.txt). See [RELEASING.md](RELEASING.md) for the automated GitHub release process. The first release is `v0.1.0`.

## Icon credits

Selected DaVinci Resolve icons are by Jean Almeida, adapted from the [DaVinci Resolve Nodes Icon Pack](https://github.com/JEAN-ALMEIDA-CZO/DaVinciResolveNodesIconPack). The pack's README states that it is MIT licensed. The pack targets Ulanzi Studio, but the PNG artwork is reused here for OpenDeck action icons.

All 70 original PNGs are bundled in `assets/icons/iconpack/`, with the pack metadata and upstream README alongside them. They are included in the packaged plugin for future action mappings; current actions continue to use the selected copies in `assets/icons/`.

Custom Undo and Redo icons were created and contributed by Martijn (DiXY); they replace the corresponding action icons. `assets/icons/blank.png` is a contributed blank icon for future use. See [CONTRIBUTING.md](CONTRIBUTING.md) for icon contribution notes.

