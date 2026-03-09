# DiagramPche :: egui

## TODO

* General:
  * [x] View → Jump to canvas origin
  * [ ] View → Toolbar
    * [ ] Source: Allow changing font size of the text editor
    * [ ] Source: Show cursor position in the text editor
    * [ ] Canvas: Change color node with color picker
    * [ ] Canvas: Change node's type via Drop-down list
  * [ ] View → Secondary canvas toolbar
    * [ ] Drag'n'Drop new node, button for each node type
    * [ ] Zoom slider, right aligned
  * Saving/loading:
    * [ ] New, Open, Save, Save as
    * [ ] Confirm unsaved exit (OS close button, alt+f4, mainmenubar option)
    * [ ] Confirm unsaved new
    * [ ] Confirm unsaved open, including loading a debug diagram or an example
    * [ ] Change window name according to opened document (incl. star for unsaved)
  * [ ] Code editor syntax highlight
* Pipeline: TOML parse → Draw to canvas / Render to SVG:
  * [x] Remove `(expecting [X, Y])` from error message (does not make sense in wider context)
  * [x] `[node].type = "text"`
  * [x] `[node].type = "rectangle"`
  * [x] `[node].type = "ellipse"`
  * [x] `[node].type = "diamond"`
  * [x] `[node].color_border`, don't forget to generalize the error message
  * [x] `[node].label_shift`, allow for variables, don't forget about zoom level
  * [x] Allow setting individual start/end path shift, keep backwards compatible
  * [ ] `[[path]].label`
  * [ ] `[[path]].label_bg`
* Canvas features:
  * [ ] Canvas zooming: anchor under mouse
  * [ ] Change the cursor type while dragging the canvas
  * [ ] Ctrl+Click to jump to node definition in text editor
* Settings window:
  * [ ] Switch between vanilla and 3rd party text editor
  * [ ] Appearance: color theme
  * [ ] Option for forcing light canvas regardless of color theme
  * [ ] Duplicate settings from app: view→grid/toolbar/toolbar2, text editor font size (so all things that can be set and saved are at one place)
* Benchmark window:
  * [ ] Do?
* Icons:
  * [ ] Main menu bar → File items
  * [ ] Toolbar → Drop-down list with node types
  * [ ] Secondary canvas toolbar → Drag'n'Drop buttons
* TOML Parser:
  * [ ] Refactor TOML parsing: do not std::move into priority queue, use pq just for determining draw order and nothing more

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
