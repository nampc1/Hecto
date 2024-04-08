#![warn(clippy::all, clippy::pedantic)]
use crate::editor::Editor;

mod editor;
mod terminal;

pub use terminal::Terminal;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
