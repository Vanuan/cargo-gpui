# cargo-gpui

Cargo subcommand to scaffold [GPUI](https://gpui.rs) apps from a starter template, with backend selection.

## Quickstart

```bash
cargo install cargo-gpui
cargo gpui new my-app
cd my-app
cargo run
```

`cargo gpui new` prompts you for a backend and a starter:

```
✔ Which GPUI backend? · gpui-unofficial
✔ Which example/starter would you like to generate? · counter-immediate-ripple
```

The generated project contains exactly the starter you picked.

## Starters

- **hello-world** (~20 lines): the minimal entity + window setup, with a single styled `div()`.
- **counter-immediate-box** (~100 lines): adds a custom `Element` (a stroked square) and a click handler wired up with `cx.notify()`.
- **counter-immediate-ripple** (~300 lines): a full counter app; actions and keybindings, a focus handle, relative/absolute layout, and a custom `RippleElement` implementing onclick feedback animation via `Element::paint` and `window.on_next_frame`. Also includes a custom titlebar and drag region (dragging and resizing aren't wired up yet).

The core idea is to demostrate GPUI's three layers: entity/window, view, and custom element. A side goal is to bridge together all the forks and variations of GPU under one roof.

## Backends

Currently only `gpui-unofficial` is available. A `gpui-ce` backend is planned, pinned to a git revision until gpu-ce has fresh published version.

## How it works

`cargo-gpui` is a thin wrapper around [`cargo-generate`](https://github.com/cargo-generate/cargo-generate), pointed at [`gpui-starter-template`](https://github.com/Vanuan/gpui-starter-template). `cargo gpui new <name>` is equivalent to running `cargo generate` against that template directly.

If you've used the older `create-gpui-app` tool, this serves the same purpose. `cargo gpui new my-app` instead of `create-gpui-app --name my-app`, but as a cargo subcommand backed by a template repo.

## License

Apache-2.0
