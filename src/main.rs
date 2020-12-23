pub mod file;
pub mod editor;
pub mod term;
pub mod event;

use editor::Editor;

fn main() -> crossterm::Result<()> {
     Editor::default().run()
}

