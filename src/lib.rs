pub mod file;
pub mod editor;
pub mod term;
pub mod event;
pub mod prompt;

use editor::Editor;

pub fn run() -> crossterm::Result<()> {
     Editor::default().run()
}

