# DiagramPche :: egui

This is part of a IMGUI paradigm research, [go back to main repo](https://github.com/RadekMocek/DP).

* Jump to:
  * [Building locally](#building-locally)
  * [Dependencies](#dependencies)
  * [Problems](#problems)

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
* [memory-stats](https://github.com/Arc-blroth/memory-stats) ([MIT](https://github.com/Arc-blroth/memory-stats/blob/main/LICENSE-MIT))
* [sysinfo](https://github.com/GuillaumeGomez/sysinfo) ([MIT](https://github.com/GuillaumeGomez/sysinfo/blob/main/LICENSE))

<br>

* [Inconsolata](https://levien.com/type/myfonts/inconsolata.html) ([SIL Open Font](https://github.com/google/fonts/blob/main/ofl/inconsolata/OFL.txt))
* [Pictogrammers Material Design Icons](https://pictogrammers.com/library/mdi/) ([Apache-2.0](https://pictogrammers.com/docs/general/license/))

## Problems

* (low) TextInput SingleLine: no equivalent to Dear ImGui's _ElideLeft_ flag
* (low) [ComboBox, DragValue, and Button not aligned properly in a horizontal layout](https://github.com/emilk/egui/issues/7412)
* (med) Canvas zooming on touchpad acts strange
* (med) I don't know how to scroll textedit to certain line or cursor, [I am not the only one](https://github.com/emilk/egui/discussions/4176)
* (med) TextEdit && TextEditAlt – cannot MW scroll while selecting text
* (high) TextEdit && TextEditAlt – vkládání symbolů s diakritikou moc nefunguje (pouze na Linux Mint, na Windows OK)
* (high) TextEdit && TextEditAlt – RAM usage on longer text
