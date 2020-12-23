pub mod action;
pub mod row;
pub mod theme;
pub mod config;

use std::{thread, sync, io::{self, Read, Write}};
use crate::{
    term::{Term, TermOp},
    file::OpenFile,
    editor::row::Row,
    editor::action::Action,
};
use std::env;
use crossterm::{
    execute, write_ansi_code, cursor, cursor::*,
    event::{KeyCode, KeyEvent, KeyModifiers, read, poll},
    style::Color, Result as TermResult,
};

#[derive(Debug)]
pub struct Editor{
    quit: bool,
    insert: bool,
    term: Term,
    cursor: Coords,
    offset: Coords,
    action: Action,
    files: Vec<OpenFile>,
    file_idx: usize,
}

impl Editor {

    pub fn run(&mut self) -> TermResult<()> {
        while self.quit != false {
            self.refresh()?;
            self.process_key()?;
        }
        Ok(())
    }

    fn refresh(&self) -> TermResult<()> {
        Term::ex(TermOp::CursorEnabled(false))?;
        Term::ex(TermOp::SetCursor(Coords::default()))?;
        if self.quit {
            Term::ex(TermOp::ClearLn)?;
        } else {
            self.draw_rows()?;
            self.draw_status()?;
            self.draw_msg()?;
            Term::ex(TermOp::SetCursor(Coords {
                x: self.cursor.x.saturating_sub(self.offset.x),
                y: self.cursor.y.saturating_sub(self.offset.y),
            }))?;
        }
        Term::ex(TermOp::CursorEnabled(true))?;
        Term::ex(TermOp::Flush)
    }

    fn draw_row(&self, row: &Row) {
        let (start, end) =
            (self.offset.x,
             self.offset.x + self.term.dims.x);
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn curr_file(&self) -> &OpenFile {
        self.files.get(self.file_idx as usize).unwrap()
    }

    fn draw_rows(&self) -> TermResult<()> {
        let t_height = self.term.dims.y;
        for row_idx in 0..t_height {
            Term::ex(TermOp::ClearLn)?;
            let curr = self.curr_file();
            if let Some(row) = curr.get(row_idx + self.offset.y) {
                self.draw_row(&row);
            } else if self.curr_file().is_empty() && row_idx == t_height / 3 {
                self.welcome();
            } else {
                println!("~\r");
            }
        }
        Ok(())
    }

    fn draw_status(&self) -> TermResult<()> {
        Term::ex(TermOp::SetBg(Color::Cyan))?;
        println!("{}\r", " ".repeat(self.term.dims.x as usize));
        Term::ex(TermOp::SetBg(Color::Reset))?;
        Ok(())
    }

    pub fn process_key(&mut self) -> TermResult<()> {
        Self::execute(Term::read_key()?)?;
        Ok(())
    }

    fn draw_msg(&self) -> TermResult<()> {
        Term::ex(TermOp::ClearLn)?;
        Ok(())
    }

    fn scroll(&mut self) {
        let curr_pos = &self.cursor;
        let (w, h): (usize, usize) = (self.term.dims.x, self.term.dims.y);
        let mut offset = &mut self.offset;
        if curr_pos.y < offset.y {
            offset.y = curr_pos.y;
        } else if curr_pos.y >= offset.y.saturating_add(h) {
            offset.y = curr_pos.y.saturating_sub(h).saturating_add(1);
        }
        if curr_pos.x < offset.x {
            offset.x = curr_pos.x;
        } else if curr_pos.x >= offset.x.saturating_add(w) {
            offset.x = curr_pos.x.saturating_sub(w).saturating_add(1);
        }
    }

    fn welcome(&self) {
        let vers: &str = env!("CARGO_PKG_VERSION");
        let msg = format!("Div {}\r", vers);
        let width = std::cmp::min(self.term.dims.y as usize, msg.len());
        println!("{}\r", &msg[..width])

    }

    fn add_file(&mut self, file: OpenFile) {
        self.files.push(file);
        self.file_idx = self.files.len();
    }

    fn del_file(&mut self, file_idx: usize) {
        self.files.remove(file_idx);
        if self.file_idx == self.files.len() + 1 {
            self.file_idx = self.files.len();
        }
    }

    pub fn execute(action: Action) -> TermResult<()> {
        use action::Direction::*;
        let mut s = std::io::stdout();
        let pos = Coords::from(cursor::position().unwrap());
        match action {
            Action::Move(loc) => match loc {
                Up(n) => execute!(s, MoveUp(n))?,
                Down(n) => execute!(s, MoveUp(n))?,
                Left(n) => execute!(s, MoveUp(n))?,
                Right(n) => execute!(s, MoveRight(n))?,
                ToIdx(loc) => execute!(s, MoveTo(loc.0, loc.1))?,
                _ => {},
            },
            Action::Quit => Term::ex(crate::term::TermOp::Exit)?,
            _ => {},
        }
        return Ok(());
    }

}

impl Default for Editor {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut files = Vec::new();
        if args.len() > 1 {
            for arg in args {
                files.push(OpenFile::new(&arg).unwrap_or_default());
            }
        } else {
            files = vec![OpenFile::default()]
        };
        let term = Term::default();
        term.init().expect("Could not init");
        Self {
            quit: false,
            files, term,
            ..Default::default()
        }
    }
}


#[derive(Default, Clone, Debug)]
pub struct Coords {
    pub x: usize, pub y: usize
}

impl Coords {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x: x as usize, y: y as usize }
    }
}

impl From<Coords> for (u16, u16) {
    fn from(coords: Coords) -> (u16, u16) {
        (coords.x as u16, coords.y as u16)
    }
}

impl From<(u16, u16)> for Coords {
    fn from((x, y): (u16, u16)) -> Self {
        Self::new(x, y)
    }
}

#[derive(Debug)]
pub enum Dir {
    Up, Down, Left, Right, Tab,
}

impl Dir {

    pub fn go(self, term: Term, amount: usize) -> TermResult<()> {
        Term::ex(TermOp::Move(self, amount as u16))?;
        Ok(())
    }
}


