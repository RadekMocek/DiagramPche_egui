# DiagramPche :: egui

This is part of a IMGUI paradigm research, [go back to main repo](https://github.com/RadekMocek/DP).

* Jump to:
  * [Building locally](#building-locally)
  * [Dependencies](#dependencies)
  * [Problems](#problems)

## TODO

* General:
  * [x] View → Jump to canvas origin
  * [x] View → Toolbar
    * [x] Source: Allow changing font size of the text editor
    * [x] Source: Show cursor position in the text editor
    * [x] Canvas: Change color node with color picker (add the parameter if it's missing in source; be aware that color def can be multiple lines if array)
    * [x] Canvas: Change node's type via Drop-down list (add the parameter if it's missing in source)
  * [x] View → Secondary canvas toolbar
    * [x] Drag'n'Drop new node, button for each node type (beware the canvas scroll and zoom)
    * [x] Zoom slider, right aligned
  * Saving/loading:
    * [ ] New, Open, Save, Save as
    * [ ] Confirm unsaved exit (OS close button, alt+f4, mainmenubar option)
    * [ ] Confirm unsaved new
    * [ ] Confirm unsaved open, including loading a debug diagram or an example
    * [ ] Change window name according to opened document (incl. star for unsaved)
  * [x] Code editor syntax highlight
* Pipeline: TOML parse → Draw to canvas / Render to SVG:
  * [x] Remove `(expecting [X, Y])` from error message (does not make sense in wider context)
  * [x] `[node].type = "text"`
  * [x] `[node].type = "rectangle"`
  * [x] `[node].type = "ellipse"`
  * [x] `[node].type = "diamond"`
  * [x] `[node].color_border`, don't forget to generalize the error message
  * [x] `[node].label_shift`, allow for variables, don't forget about zoom level
  * [x] Allow setting individual start/end path shift, keep backwards compatible
  * [x] `[[path]].label`
  * [x] `[[path]].label_bg`
* Canvas features:
  * [x] Canvas zooming: anchor under mouse
  * [x] Change the cursor type while dragging the canvas
  * [ ] Ctrl+Click to jump to node definition in text editor
* Settings window:
  * [x] Switch between vanilla and 3rd party text editor
  * [ ] Appearance: color theme
  * [ ] Option for forcing light canvas regardless of color theme
  * [x] Duplicate settings from app: view→grid/toolbar/toolbar2, text editor font size (so all things that can be set are at one place)
* Benchmark window:
  * [ ] Do?
* Icons:
  * [x] Main menu bar → File items
  * [x] Toolbar → Drop-down list with node types
  * [x] Secondary canvas toolbar → Drag'n'Drop buttons
* TOML Parser:
  * [x] Refactor TOML parsing: if we get parsing error, do not make the whole diagram disappear

## Building locally

```bash
# Install Rust, follow https://rust-lang.org/learn/get-started/

# Navigate to the root directory of this repository

cargo build --release
```

## Dependencies

* [egui](https://github.com/emilk/egui) ([MIT](https://github.com/emilk/egui/blob/main/LICENSE-MIT))
* [toml_edit](https://github.com/toml-rs/toml/tree/main/crates/toml_edit) ([MIT](https://github.com/toml-rs/toml/blob/main/LICENSE-MIT))
* [svg](https://github.com/bodoni/svg) ([MIT](https://github.com/bodoni/svg/blob/main/LICENSE.md))
* [open](https://github.com/Byron/open-rs) ([MIT](https://github.com/Byron/open-rs/blob/main/LICENSE.md))
* [rfd](https://github.com/PolyMeilex/rfd) ([MIT](https://github.com/PolyMeilex/rfd/blob/master/LICENSE))
* [showfile](https://github.com/jf2048/showfile) ([MIT](https://github.com/jf2048/showfile/blob/main/COPYING))
* [egui_code_editor](https://github.com/p4ymak/egui_code_editor) ([MIT](https://github.com/p4ymak/egui_code_editor/blob/main/LICENSE))
* [IconFontCppHeaders](https://github.com/juliettef/IconFontCppHeaders) ([zlib](https://github.com/juliettef/IconFontCppHeaders/blob/main/licence.txt))
* [const_format](https://github.com/rodrimati1992/const_format_crates/) ([zlib](https://github.com/rodrimati1992/const_format_crates/blob/master/LICENSE-ZLIB.md))

<br>

* [Inconsolata](https://levien.com/type/myfonts/inconsolata.html) ([SIL Open Font](https://github.com/google/fonts/blob/main/ofl/inconsolata/OFL.txt))
* [Pictogrammers Material Design Icons](https://pictogrammers.com/library/mdi/) ([Apache-2.0](https://pictogrammers.com/docs/general/license/))

## Problems

* Error highlight breaks on tab
* TextInput SingleLine: no equivalent to Dear ImGui's _ElideLeft_ flag
* Canvas zooming on touchpad acts strange
* I don't know how to scroll textedit to certain line or cursor, [I am not the only one](https://github.com/emilk/egui/discussions/4176)
* <https://github.com/emilk/egui/issues/7412>
