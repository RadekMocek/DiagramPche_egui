# DiagramPche :: egui

## TODO

* General:
  * [x] View → Jump to canvas origin
  * Saving/loading:
    * [ ] New, Open, Save, Save as
    * [ ] Confirm unsaved exit
    * [ ] Confirm unsaved open, including loading a debug diagram or an example
    * [ ] Change window name according to opened document (incl. star for unsaved)
  * [ ] Code editor syntax highlight
  * [ ] Klávesové zkratky (<kbd>Alt</kbd> pro navigaci v MainMenuBar, <kbd>Ctrl</kbd>+<kbd>S</kbd> pro uložení atd.)
* Canvas features:
  * [ ] Canvas zooming: anchor under mouse
  * [ ] Change the cursor type while dragging the canvas
  * [ ] Ctrl+Click to jump to node definition in editor

## Problems

* Error highlight breaks:
  * On tab
  * On too thin textedit because it wraps lines!
* TextInput SingleLine: no equivalent to Dear ImGui's _ElideLeft_ flag (?)
* Canvas zooming on touchpad not ideal

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
