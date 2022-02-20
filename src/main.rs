use std::{
    fs::File,
    io::{self, BufReader, Write},
    process::exit,
    thread,
};

use crossbeam_channel::Receiver;
use ropey::Rope;

mod tako;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Normal,
    Insert,
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
struct Editor {
    text: Rope,
    mode: Mode,
    position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UpdateResult {
    Redraw,
    DontRedraw,
    Exit,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            text: Rope::new(),
            mode: Mode::Normal,
            position: Position { row: 0, col: 0 },
        }
    }

    pub fn draw(&self, term: &mut console::Term) -> io::Result<()> {
        let (height, width) = term.size();

        // term.clear_screen()?;

        let mut y = 0;
        for line in self.text.lines() {
            term.move_cursor_to(0, y)?;
            write!(term, "{}", line)?;
            y += 1;
        }

        let tako_pos = Position {
            col: width as usize - 43,
            row: height as usize - 19,
        };

        tako::draw_tako(term, tako_pos)?;

        term.move_cursor_to(0, height as usize)?;

        let pos_str = format!(" {},{} ", self.position.col, self.position.row);
        let mode_str = format!(" {} ", self.mode.to_string());

        write!(
            term,
            "{}{}{}",
            console::style(" ".repeat(width as usize - pos_str.len() - mode_str.len()))
                .bg(console::Color::Cyan),
            console::style(pos_str).bg(console::Color::Cyan),
            console::style(mode_str).bg(console::Color::Blue),
        )?;

        term.move_cursor_to(self.position.col, self.position.row)?;
        term.flush()
    }

    fn rope_index_from_pos(&self) -> usize {
        let line = self.position.row;
        self.text.line_to_char(line) + self.position.col
    }

    pub fn update(&mut self, key: console::Key, term_size: Size) -> UpdateResult {
        match key {
            console::Key::Char('i' | 'a' | 'I' | 'A') if self.mode == Mode::Normal => {
                self.mode = Mode::Insert;
                UpdateResult::Redraw
            }
            console::Key::Escape => {
                self.mode = Mode::Normal;
                UpdateResult::Redraw
            }
            console::Key::Char('h') if self.mode == Mode::Normal && self.position.col > 0 => {
                self.position.col -= 1;
                UpdateResult::Redraw
            }
            console::Key::Char('k') if self.mode == Mode::Normal && self.position.row > 0 => {
                self.position.row -= 1;
                UpdateResult::Redraw
            }
            console::Key::Char('j')
                if self.mode == Mode::Normal && self.position.row < term_size.height - 2 =>
            {
                self.position.row += 1;
                UpdateResult::Redraw
            }
            console::Key::Char('l')
                if self.mode == Mode::Normal && self.position.col < term_size.width - 1 =>
            {
                self.position.col += 1;
                UpdateResult::Redraw
            }
            console::Key::Char('q') if self.mode == Mode::Normal => UpdateResult::Exit,
            console::Key::Enter if self.mode == Mode::Insert => {
                self.text
                    .insert_char(self.text.line_to_char(self.position.row + 1), '\n');
                self.position.row += 1;
                self.position.col = 0;
                UpdateResult::Redraw
            }
            console::Key::Char(char) if self.mode == Mode::Insert => {
                self.text.insert_char(self.rope_index_from_pos(), char);
                self.position.col += 1;
                UpdateResult::Redraw
            }
            _ => UpdateResult::DontRedraw,
        }
    }
}

fn main() -> Result<(), io::Error> {
    // let fname = std::env::args().into_iter().nth(1).unwrap();

    // let mut text = Rope::from_reader(BufReader::new(File::open(fname)?))?;

    let mut term = console::Term::buffered_stdout();

    let mut editor = Editor::new();

    term.clear_screen()?;

    editor.draw(&mut term)?;

    loop {
        let key = term.read_key()?;
        let (height, width) = term.size();
        let term_size = Size {
            width: width as usize,
            height: height as usize,
        };

        match editor.update(key, term_size) {
            UpdateResult::Redraw => editor.draw(&mut term)?,
            UpdateResult::DontRedraw => {}
            UpdateResult::Exit => break,
        }
    }

    term.clear_screen()?;
    term.flush()?;

    Ok(())
}
