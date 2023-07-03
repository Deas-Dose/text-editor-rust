#![warn(clippy::all, clippy::pedantic)]
mod editor;


use editor::Editor;

// main function
fn main() {
    Editor::default().run();
}

