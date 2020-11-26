#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::clippy::used_underscore_binding,
    clippy::clippy::cast_sign_loss,
)]

pub mod file;
pub mod editor;
pub mod term;

use editor::Editor;

fn main() -> crossterm::Result<()> {
     Editor::default().run()
}

