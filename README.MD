# DiagramPche :: egui

## TODO

* [ ] Test all error message scenarios
* [ ] Tooltip showing node's id on hover
* [x] SVG export ability
* [ ] SVG fix: colors
* [ ] SVG fix: wrong dimensions
* [ ] SVG fix: dependant on zoom_level
* [ ] SVG export proper dialog

## Problems

* Error highlight breaks:
  * On tab
  * On too thin textedit because it wraps lines!

## Dependencies

* [egui](https://github.com/emilk/egui) ([MIT](https://github.com/emilk/egui/blob/main/LICENSE-MIT))
* [toml_edit](https://github.com/toml-rs/toml/tree/main/crates/toml_edit) ([MIT](https://github.com/toml-rs/toml/blob/main/LICENSE-MIT))
* [Inconsolata](https://levien.com/type/myfonts/inconsolata.html) ([SIL Open Font](https://github.com/google/fonts/blob/main/ofl/inconsolata/OFL.txt))
* [svg](https://github.com/bodoni/svg) ([MIT](https://github.com/bodoni/svg/blob/main/LICENSE.md))
* [open](https://github.com/Byron/open-rs) ([MIT](https://github.com/Byron/open-rs/blob/main/LICENSE.md))
* []() ([]())
* []() ([]())

## Building locally

```bash
cargo build --release
```
