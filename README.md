# Ferro Solitaire

Klondike, FreeCell and Spider Solitaire built with [FerroUI](https://github.com/wieslawsoltes/FerroUI), a Rust
UI framework. It runs natively on the desktop and in the browser (WebAssembly).
The game is a port of an open-source .NET Solitaire application (see [NOTICE.md](NOTICE.md)): its rules,
XAML pages, vector playing cards, title artwork and fonts are retained, with animated card movement, timed
dealing, a startup fade and animated gold trays.

**Play it in the browser:** https://ferroui.github.io/Solitaire/

![Klondike](docs/images/Klondike.png)

## Repository layout

| Path | Content |
|---|---|
| `Solitaire/` | the game (`ferro-solitaire`): models, view models, controls, XAML pages and the desktop entry point |
| `Solitaire.Browser/` | the browser host (`ferro-solitaire-browser`): the WebAssembly entry point and its host page in `wwwroot/` |
| `FerroUI/` | FerroUI as a git submodule; the game builds against its crates |
| `docs/` | porting notes and screenshots |

The repository follows the layout of FerroUI applications described in
[FerroUI/docs/apps.md](https://github.com/wieslawsoltes/FerroUI/blob/main/docs/apps.md): the workspace
dependencies and profiles of `Cargo.toml` are kept in step with the submodule by
`FerroUI/scripts/apps/sync-workspace.py`, and the CI is FerroUI's shared application workflow.

## Run

```sh
git clone --recurse-submodules https://github.com/FerroUI/Solitaire.git
cd Solitaire
cargo run --profile fast-release -p ferro-solitaire
```

Rust 1.89 or newer is required, with the platform prerequisites of FerroUI: the Xcode command line tools
on macOS; a C/C++ compiler, pkg-config, fontconfig and FreeType development packages on Linux; the MSVC
C++ build tools on Windows. The first build downloads Skia binaries and builds HarfBuzz.

Open a game directly with `-- --klondike`, `-- --freecell` or `-- --spider`.

### Browser

With the pinned browser toolchain of FerroUI (`FerroUI/scripts/browser/setup.sh`):

```sh
source ~/browser-tools/env.sh
FERROUI_BROWSER_WORKSPACE=$PWD FerroUI/scripts/build-browser.sh ferro-solitaire-browser
```

Serve `target/browser/ferro-solitaire-browser` with any static web server. In the browser, saved games
last as long as the page is open.

## Play

- Drag a face-up card or legal run onto its destination, or click a card and then its destination.
- Double-click or right-click a card to send it to its foundation. **Auto move** repeatedly moves eligible
  cards to foundations.
- Click the stock or **Draw** to turn Klondike cards. Spider's **Deal row** requires every column to
  contain a card.
- **Hint** highlights a legal move. **Undo** restores a move, card reveal, stock turn or complete Spider
  run. **Restart** replays the same deal; **New game** shuffles another deck.
- Settings select Klondike draw one or draw three, and Spider one, two or four suits. Changes apply to
  the next new game.
- **Space** draws, **H** shows a hint, **A** auto-moves, **Ctrl/Cmd+Z** undoes, **Ctrl/Cmd+N** starts a
  new game, and **Escape** clears the selection or dismisses a dialog.

On the desktop, settings, current games, undo history and statistics are saved in
`FerroSolitaire/state.json` under the local application data directory (`FERRO_SOLITAIRE_DATA_DIR`
overrides it). Invalid or unsupported saves are reported in the status bar and are not overwritten.

## Test

```sh
cargo test --locked -p ferro-solitaire --no-default-features
cargo test --locked -p ferro-solitaire --features headless --test ui --test animations
```

The rule tests cover deals, suit distributions, legal runs, foundations, stock recycling, scoring,
victories, exact undo and save validation. The UI test loads every page with the real XAML loader and
Skia renderer, invokes commands, and simulates mouse dragging and click-to-move. The animation test steps
a manual clock and checks intermediate rendered frames. Set `FERRO_SOLITAIRE_CAPTURE_DIR` to export the
rendered pages.

See [the porting notes](docs/PORTING.md) for the source mapping, intentional fixes and visual
differences.

## License

MIT, see [LICENSE](LICENSE). The original work's copyright and license are in [NOTICE.md](NOTICE.md).
