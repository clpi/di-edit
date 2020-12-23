use std::io::{self, stdout, stdin, Stdout, Read, Write};
use crossterm::{
    tty::IsTty,
    cursor::{MoveTo, self},
    write_ansi_code, execute, queue,
    Command, Result as TermResult,
    terminal::{self, LeaveAlternateScreen, EnterAlternateScreen, Clear, ClearType, ScrollUp, ScrollDown},
    event::{Event, KeyEvent, read},
    style::{Color, SetForegroundColor, SetBackgroundColor, SetColors},
};

use crate::{
    editor::{Coords, Dir, action::Action},

};

#[derive(Debug)]
pub struct Term {
    pub dims: Coords,
    _stdout: io::Stdout,
}

#[derive(Debug)]
pub enum TermOp {
    Clear,
    ClearLn,
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
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), terminal::SetTitle("dd"))?;
        Ok ( Self { dims, _stdout: io::stdout() })
    }

    pub fn init(&self) -> TermResult<()> {
        Self::ex(TermOp::Enter)?;
        Ok(())
    }


    pub fn ex(operation: TermOp) -> TermResult<()> {
        let mut so: io::Stdout = io::stdout();
        use TermOp::*;
        match operation {
            Enter => {
                terminal::enable_raw_mode()?;
                crossterm::queue!(so, terminal::Clear(ClearType::All))?;
                execute!(so, EnterAlternateScreen)?
            },
            Clear => execute!(so, terminal::Clear(ClearType::All))?,
            ClearLn => execute!(so, terminal::Clear(ClearType::CurrentLine))?,
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
            },
        }
        Ok(())
    }


    pub fn size(self) -> Coords {
        self.dims
    }

    pub fn read_key() -> TermResult<Action> {
        loop {
            if let Event::Key(key_event) = read()? {
                let act = Action::from((key_event.code, key_event.modifiers));
                return Ok(act);
            }
        }
    }

    pub fn colors() -> usize {
        if std::env::var("COLORTERM").unwrap_or_else(|_| "".into()).eq("truecolor".into()) {
            return 24;
        } else if let Ok(inf) = term::terminfo::TermInfo::from_env() {
            if let Some(num) = inf.numbers.get("colors") {
                match num {
                    &256 => return 256,
                    _ => return 16,
                }
            } else { return 16 }
        } else { return 16  }
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
