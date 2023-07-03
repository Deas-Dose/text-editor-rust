#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

// main function
fn main() {
    let editor = Editor::default();
    editor.run();
}

