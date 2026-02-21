# DiagramPche :: egui

## TODO

* [ ] Test all error message scenarios
* [ ] Tooltip showing node's id on hover
* [x] SVG export ability
* [x] SVG export proper dialog

## Problems

* Error highlight breaks:
  * On tab
  * On too thin textedit because it wraps lines!
* TextInput SingleLine: no equivalent to Dear ImGui's _ElideLeft_ flag (?)

## Dependencies

* [egui](https://github.com/emilk/egui) ([MIT](https://github.com/emilk/egui/blob/main/LICENSE-MIT))
* [toml_edit](https://github.com/toml-rs/toml/tree/main/crates/toml_edit) ([MIT](https://github.com/toml-rs/toml/blob/main/LICENSE-MIT))
* [Inconsolata](https://levien.com/type/myfonts/inconsolata.html) ([SIL Open Font](https://github.com/google/fonts/blob/main/ofl/inconsolata/OFL.txt))
* [svg](https://github.com/bodoni/svg) ([MIT](https://github.com/bodoni/svg/blob/main/LICENSE.md))
* [open](https://github.com/Byron/open-rs) ([MIT](https://github.com/Byron/open-rs/blob/main/LICENSE.md))
* [rfd](https://github.com/PolyMeilex/rfd) ([MIT](https://github.com/PolyMeilex/rfd/blob/master/LICENSE))
* [showfile](https://github.com/jf2048/showfile) ([MIT](https://github.com/jf2048/showfile/blob/main/COPYING))

## Building locally

```bash
cargo build --release
```
