use std::convert::TryFrom;
use crossterm::{
    event::{KeyCode, KeyModifiers},
};

pub enum KeyAction {
    Move(Direction),
    MoveText(Target, Direction),
    Scroll(Direction),
    Delete(Direction),
    Select(Direction),
    Input(String),
    Unregistered((KeyCode, KeyModifiers)),
    SwitchBuffer(RelativeLocation),
    SwitchTab(RelativeLocation),
    Copy(Target),
    Paste,

}

pub enum Direction {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
    To(Location),
}
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

pub enum Location {
    End(Target),
    Beginning(Target),
    Index(u8),
}

pub enum RelativeLocation {
    Next,
    Previous,
    DiffIndex(i8),
}

impl KeyAction {

}

impl From<(KeyCode, KeyModifiers)> for KeyAction {
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
                    KeyCode::Up => Self::Scroll(Up(5)),
                    KeyCode::Down => Self::Scroll(Down(5)),
                    KeyCode::Left => Self::Scroll(Left(5)),
                    KeyCode::Right => Self::Scroll(Right(5)),
                    KeyCode::Tab => return Self::SwitchTab(Previous),
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

