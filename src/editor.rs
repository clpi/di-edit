pub mod row;

use crate::{
    term::{Term, TermOp},
    file::OpenFile,
    editor::row::Row,
};
use std::{
    io::{self, stdout, Read, Write}, env
};
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, Event, KeyModifiers},
    style::{Color, Colors, Attribute, SetBackgroundColor, SetForegroundColor},
    ErrorKind, Result as TermResult,
};
use regex::Regex;


pub struct Editor{
    quit: bool,
    insert: bool,
    term: Term,
    cursor: Coords,
    offset: Coords,
    files: Vec<OpenFile>,
    file_idx: usize,
}

impl Editor {

    pub fn run(&mut self) -> TermResult<()> {
        loop {
            if let Err(err) = self.refresh() {
                panic!(err);
            }
            if self.quit { break; }
            if let Err(error) = self.process_key() {
                panic!(error);
            }
        }
        Ok(())
    }

    fn refresh(&self) -> TermResult<()> {
        Term::execute(TermOp::CursorEnabled(false))?;
        Term::execute(TermOp::SetCursor(Coords::default()))?;
        if self.quit {
            Term::execute(TermOp::Clear)?;
        } else {
            self.draw_rows()?;
            self.draw_status()?;
            self.draw_msg()?;
            Term::execute(TermOp::SetCursor(Coords {
                x: self.cursor.x.saturating_sub(self.offset.x),
                y: self.cursor.y.saturating_sub(self.offset.y),
            }))?;
        }
        Term::execute(TermOp::CursorEnabled(true))?;
        Term::execute(TermOp::Flush)
    }

    fn process_key(&mut self) -> TermResult<()> {
        let key_event = Term::read_key()?;
        use KeyCode::*;
        match key_event.code {
            Up | Char('j')
            | Down | Char('h')
            | Left | Char('k')
            | Right | Char('l')
            | PageUp | Char('d')
            | PageDown | Char('e')
            | End | Char('f')
            | Home | Char('s') => {self.move_cursor(key_event)?;},
            Char('q') => { self.quit = true; },
            Char('i') => { self.insert = true; },
            Char(c) => {
                if c.is_control() {
                    println!("{:?}\r", c as u8);
                } else {
                    println!("{:?} ({})\r", c as u8, c);
                }
            },
            _ => {println!("{:?}\r", key_event.code);},
        };
        self.scroll();
        Ok(())
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
            Term::execute(TermOp::Clear)?;
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
        let spc = " ".repeat(self.term.dims.x as usize);
        Term::execute(TermOp::SetBg(Color::Cyan))?;
        println!("{}\r", spc);
        // Term::reset_bg();
        Ok(())
    }

    fn draw_msg(&self) -> TermResult<()> {
        Term::execute(TermOp::Clear)?;
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

    fn move_cursor(&mut self, event: KeyEvent) -> TermResult<()> {
        use KeyCode::*;
        let mut new: Coords = self.cursor.clone();
        if event.modifiers.is_empty() {
            match event.code {
                Up | Char('k') => {Dir::Up.go(1)?; new.y+= 1;},
                Down | Char('j') => {Dir::Down.go(1)?; new.y -=1 },
                Left | Char('h')=> {Dir::Left.go(1)?; new.x -=1;},
                Right | Char('l')=> {Dir::Right.go(1)?; new.x += 1;},
                PageUp => {Term::execute(TermOp::Scroll(Dir::Up, 1))?;},
                PageDown => {Term::execute(TermOp::Scroll(Dir::Up, 1))?},
                Home => new.x = 0,
                End => new.x = self.curr_file()
                    .get(self.cursor.x as usize)
                    .unwrap().len(),
                _ => {},
            };
        } else if event.modifiers.contains(KeyModifiers::CONTROL) {
            use TermOp::Scroll;
            match event.code {
                Char('j') => {Term::execute(Scroll(Dir::Down, 1))?;},
                Char('k') => {Term::execute(Scroll(Dir::Up, 1))?;},
                _ => new.y = 0,
            }
        }
        let width = match self.curr_file().get(self.cursor.y) {
            Some(row) => row.len(),
            None => 0,
        };
        if self.cursor.x > width { new.x = width }
        self.cursor = new;
        Ok(())
    }

    fn welcome(&self) {
        let vers: &str = env!("CARGO_PKG_VERSION");
        let msg = format!("Div {}\r", vers);
        let width = std::cmp::min(self.term.dims.y as usize, msg.len());
        println!("{}\r", &msg[..width])

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
        Self {
            term: Term::default(),
            quit: false,
            files,
            ..Default::default()
        }
    }
}

#[derive(Default, Clone)]
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

pub enum Dir {
    Up, Down, Left, Right, Tab,
}

impl Dir {

    pub fn go(self, amount: usize) -> TermResult<()> {
        Term::execute(TermOp::Move(self, amount as u16))?;
        Ok(())
    }
}
