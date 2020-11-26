use std::io::{self, stdout, Write};
use crossterm::{
    cursor::{self, MoveTo}, execute, Result as TermResult,
    terminal::{ScrollUp, ScrollDown},
    terminal::{self, ClearType, LeaveAlternateScreen, EnterAlternateScreen},
    event::{self, Event, KeyEvent, KeyCode, KeyModifiers, read},
    style::{Color, Colors, SetForegroundColor, SetBackgroundColor},
};
use cursor::MoveUp;
use crate::editor::{Coords, Dir};

pub struct Term {
    pub dims: Coords,
    _stdout: io::Stdout,
}

pub enum TermOp {
    Clear,
    Enter,
    Exit,
    Flush,
    CursorEnabled(bool),
    SetCursor(Coords),
    Move(Dir, u16),
    SetBg(Color),
    SetFg(Color),
    Scroll(Dir, u16),
}

impl Term {

    pub fn new() -> TermResult<Self> {
        let dims: Coords = terminal::size().unwrap_or_default().into();
        Self::ex(TermOp::Enter)?;
        Ok ( Self { dims, _stdout: stdout() })
    }

    pub fn ex(operation: TermOp) -> TermResult<()> {
        let mut so: io::Stdout = stdout();
        use TermOp::*;
        match operation {
            Enter => {
                terminal::enable_raw_mode()?;
                execute!(so, EnterAlternateScreen)?
            },
            Clear => execute!(so, LeaveAlternateScreen)?,
            Exit => {
                execute!(so, LeaveAlternateScreen)?;
                terminal::disable_raw_mode()?;
            }
            CursorEnabled(true) => execute!(so, cursor::Show)?,
            CursorEnabled(false) => execute!(so, cursor::Hide)?,
            Flush => so.flush()?,
            SetCursor(c) => execute!(so, MoveTo(c.x as u16, c.y as u16))?,
            SetBg(color) => execute!(so, SetBackgroundColor(color))?,
            SetFg(color) => execute!(so, SetForegroundColor(color))?,
            Move(dir, amt) => match dir {
                Dir::Up => execute!(so, cursor::MoveToPreviousLine(amt))?,
                Dir::Down => execute!(so, cursor::MoveToNextLine(amt))?,
                Dir::Left => execute!(so, cursor::MoveLeft(amt))?,
                Dir::Right => execute!(so, cursor::MoveRight(amt))?,
                _ => (),
            },
            Scroll(dir, amt) => match dir {
                Dir::Up => execute!(so, ScrollUp(amt))?,
                Dir::Down => execute!(so, ScrollDown(amt))?,
                _ => (),
            }
            _ => {}
        }
        Ok(())
    }


    pub fn size(self) -> Coords {
        self.dims
    }

    pub fn read_key() -> TermResult<KeyEvent> {
        loop {
            if let Event::Key(key_event) = read()? {
                return Ok(key_event);
            }
        }
    }

}

impl Default for Term {
    fn default() -> Self {
        let dims: Coords = terminal::size().unwrap_or_default().into();
        Self {
            dims, _stdout: stdout(),
        }
    }
}
