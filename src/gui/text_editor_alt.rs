use crate::App;
use egui_code_editor::{CodeEditor, ColorTheme, Completer, Syntax};
use std::collections::BTreeSet;

// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---

impl App {
    pub(super) fn gui_text_editor_alt(&mut self, ui: &mut egui::Ui) {
        let text_edit_output = CodeEditor::default()
            .id_source("source_alt")
            .with_fontsize(self.source_font_size as f32)
            .with_theme(self.alt_editor_config.palette)
            .with_syntax(self.alt_editor_config.syntax.clone())
            .with_numlines(false)
            .show_with_completer(ui, &mut self.source, &mut self.alt_editor_config.completer);

        self.textedit_error_highlight(ui, &text_edit_output.response);

        self.textedit_update_cursor_position_info(&text_edit_output.cursor_range);

        if text_edit_output.response.changed() {
            self.is_source_dirty = true;
        }
    }
}

// --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---

pub struct AltEditorConfig {
    syntax: Syntax,
    palette: ColorTheme,
    completer: Completer,
}

impl Default for AltEditorConfig {
    fn default() -> Self {
        Self {
            syntax: Self::get_syntax(),
            palette: Self::get_palette_light(),
            completer: Completer::new_with_syntax(&Self::get_syntax()),
        }
    }
}

impl AltEditorConfig {
    fn get_syntax() -> Syntax {
        Syntax {
            language: "NSOLD",
            case_sensitive: true,
            comment: "#",
            comment_multiline: ["/*", "*/"], // Not valid, TOML does not have multiline comments, but I have to give it something
            quotes: BTreeSet::from(['\'', '"']),
            hyperlinks: BTreeSet::from(["http"]),
            keywords: BTreeSet::from(["variables", "node", "path"]),
            types: BTreeSet::from([
                "color",
                "color_border",
                "end",
                "ends",
                "label",
                "label_bg",
                "label_pos",
                "label_shift",
                "pivot",
                "points",
                "shift",
                "size",
                "start",
                "tips",
                "type",
                "value",
                "xy",
                "z",
            ]),
            special: BTreeSet::new(),
        }
    }

    fn get_palette_light() -> ColorTheme {
        ColorTheme {
            name: "DiagramPchePaletteLight",
            dark: false,
            bg: "#ffffff",
            cursor: "#000000",
            selection: "#bfbfbf",
            comments: "#879493",
            functions: "#ff0000",
            keywords: "#373dc2",
            literals: "#24292f",
            numerics: "#a02020",
            punctuation: "#24292f",
            strs: "#4d5901",
            types: "#107f76",
            special: "#ff0000",
        }
    }
}
