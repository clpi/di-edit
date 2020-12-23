use std::io::{self, Read, Write};
use crate::{
    editor::{Coords, Dir},
    term::Term,
};
use crossterm::{
    execute, write_ansi_code, Result as TermResult,
    cursor::{MoveTo, MoveUp, MoveDown, MoveLeft, MoveRight, self},
};
use std::convert::TryFrom;
use crossterm::{
    event::{KeyCode, KeyModifiers},
};

#[derive(Debug)]
pub enum Action {
    Move(Direction),
    MoveText(Target, Direction),
    Scroll(Direction),
    Delete(Direction),
    Select(Direction),
    Input(String),
    Unregistered((KeyCode, KeyModifiers)),
    SwitchBuffer(RelativeLocation),
    SwitchTab(RelativeLocation),
    Newline(Direction),
    OpenFile(std::fs::File),
    DelFile(std::fs::File),
    Quit,
    Copy(Target),
    Paste,

}

#[derive(Debug)]
pub enum Direction {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
    ToIdx((u16, u16)),
    To(Location),
}

#[derive(Debug)]
pub enum Target {
    Word,
    Document,
    Line,
    Bracketed,
    Parenthesis,
    SingleQuotes,
    DoubleQuotes,
    Backticks,
    Selection,
    Char(char),
    InputWord(String),
    Region(u32, u32),
}

#[derive(Debug)]
pub enum Location {
    End(Target),
    Beginning(Target),
    Index(u8),
}

#[derive(Debug)]
pub enum RelativeLocation {
    Next,
    Previous,
    DiffIndex(i8),
}

impl Action {

    pub fn execute(self) -> TermResult<()> {
        use Action::*;
        use Location::*;
        use Direction::*;
        let mut s = std::io::stdout();
        let pos = Coords::from(cursor::position().unwrap());
        match self {
            Move(loc) => match loc {
                Up(n) => execute!(s, MoveUp(n))?,
                Down(n) => execute!(s, MoveUp(n))?,
                Left(n) => execute!(s, MoveUp(n))?,
                Right(n) => execute!(s, MoveRight(n))?,
                ToIdx(loc) => execute!(s, MoveTo(loc.0, loc.1))?,
                _ => {},
            },
            Quit => Term::ex(crate::term::TermOp::Exit)?,
            _ => {},
        }
        return Ok(());
    }
}

impl From<(KeyCode, KeyModifiers)> for Action {
    fn from((key, kmod): (KeyCode, KeyModifiers)) -> Self {
        use Direction::{Up, Down, Left, Right, To};
        use Location::{End, Beginning};
        use RelativeLocation::{Next, Previous};
        use Target::{Line, Document};
        if kmod.eq(&KeyModifiers::NONE) {
            match key {
                KeyCode::Up => return Self::Move(Up(1)),
                KeyCode::Down => return Self::Move(Down(1)),
                KeyCode::Right => return Self::Move(Right(1)),
                KeyCode::Left => return Self::Move(Left(1)),
                KeyCode::End => return Self::Move(To(End(Line))),
                KeyCode::Home => return Self::Move(To(Beginning(Line))),
                KeyCode::PageUp => return Self::Scroll(Up(5)),
                KeyCode::PageDown => return Self::Scroll(Down(5)),
                KeyCode::Delete => return Self::Delete(Left(1)),
                KeyCode::Backspace => return Self::Delete(Left(1)),

                KeyCode::Char(c) => return Self::Input(c.to_string()),

                _ => return Self::Unregistered((key, kmod)),
            }
        } else if kmod.contains(KeyModifiers::CONTROL) {
            if kmod.contains(KeyModifiers::SHIFT) {
                match key {
                    KeyCode::Tab => return Self::SwitchTab(Next),
                    KeyCode::Enter => return Self::Newline(Up(1)),
                    _ => return Self::Unregistered((key, kmod)),
                }
            } else if kmod.contains(KeyModifiers::ALT) {
                match key {
                    _ => return Self::Unregistered((key, kmod)),
                }
            } else {
                match key {
                    KeyCode::Char('j') => Self::Move(Down(1)),
                    KeyCode::Char('k') => Self::Move(Up(1)),
                    KeyCode::Char('h') => Self::Move(Left(1)),
                    KeyCode::Char('l') => Self::Move(Right(1)),
                    KeyCode::Char('q') => Self::Quit,
                    KeyCode::Up => Self::Scroll(Up(5)),
                    KeyCode::Down => Self::Scroll(Down(5)),
                    KeyCode::Left => Self::Scroll(Left(5)),
                    KeyCode::Right => Self::Scroll(Right(5)),
                    KeyCode::Tab => return Self::SwitchTab(Previous),
                    KeyCode::Enter => return Self::Newline(Down(1)),
                    _ => return Self::Unregistered((key, kmod)),
                }
            }
        } else if kmod.contains(KeyModifiers::ALT) {
            match key {
                _ => return Self::Unregistered((key, kmod)),
            }
        } else if kmod.contains(KeyModifiers::SHIFT) {
            match key {
                KeyCode::PageUp => Self::Move(To(Beginning(Document))),
                KeyCode::PageDown => Self::Move(To(End(Document))),
                KeyCode::Char(c) => Self::Input(c.to_string().to_uppercase()),
                _ => return Self::Unregistered((key, kmod)),
            }
        } else {
            return Self::Unregistered((key, kmod));
        }
    }
}

