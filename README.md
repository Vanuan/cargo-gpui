# cargo-gpui

Cargo subcommand to scaffold [GPUI](https://gpui.rs) apps from a starter template, with backend selection.

## Quickstart

```bash
cargo install cargo-gpui --locked
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

`cargo-gpui` is a thin wrapper around [`cargo-generate`](https://github.com/cargo-generate/cargo-generate), pointed at [gpui-starter-template](https://github.com/Vanuan/gpui-starter-template). `cargo gpui new <name>` is equivalent to running `cargo generate` against that template directly.

## Prior art

[`create-gpui-app`](https://github.com/zed-industries/create-gpui-app) is the original GPUI scaffolding tool, inspired by `create-react-app`. The main difference: its examples are bundled into the crate itself, so changing which example gets generated means publishing a new version of the crate. `cargo-gpui` separates the tool from the templates, living in its own repo and updated independently. That separation also leaves room for `cargo-gpui` to grow beyond `new`. For example, a `cargo gpui hot` subcommand for running an app with hot reload.

The starters here are written against [gpui.rs](https://www.gpui.rs)'s examples and the examples bundled in zed-industries/zed's `gpui` crate. There's a lot more in there than three starters cover. Eventually it would be worth consuming more of it directly, either as an easily-switchable showcase or as a single package of multiple runnable example crates.

Other templates and ecosystem projects worth knowing about:

- [create-gpui-docview](https://github.com/blacktop/create-gpui-docview) — a template specifically for text-file viewers, a starting point for a simple text editor.
- [gpui-starter](https://github.com/xhofe/gpui-starter) (xhofe) — a fuller-featured template with an app shell, theming, and pre-built components.
- [gpui-component](https://github.com/longbridge/gpui-component) (longbridge) — probably the most widely used GPUI component library, tracking upstream zed-bundled `gpui`. A `gpui-component`-wired starter is a natural addition to `cargo gpui new`.
- [gpui-hooks](https://crates.io/crates/gpui-hooks) — React-style hooks (`use_state`, `use_effect`, `use_memo`) for GPUI.
- [gpui-router](https://github.com/justjavac/gpui-router) — routing for GPUI apps, inspired by React Router.
- [awesome-gpui](https://github.com/zed-industries/awesome-gpui) (zed-industries) — curated index of GPUI projects, libraries, and examples.


If you've used the older `create-gpui-app` tool, this serves the same purpose. `cargo gpui new my-app` instead of `create-gpui-app --name my-app`, but as a cargo subcommand backed by a template repo.

## License

Apache-2.0
