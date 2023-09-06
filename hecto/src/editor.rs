use crate::document::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use std::io::{self, Write};
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{e}");
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    current_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                die(&err);
            }
            if let Err(err) = self.process_key_press() {
                die(&err);
            }
            if self.should_quit {
                break;
            }
        }
    }
    pub fn default() -> Self {
        let terminal = Terminal::default().expect("Failed to initialize terminal");
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            Document::open(&args[1]).unwrap_or_default()
        } else {
            Document::default()
        };
        Self {
            should_quit: false,
            terminal,
            current_position: Position::default(),
            offset: Position::default(),
            document,
        }
    }
    fn process_key_press(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Right | Key::Left => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }
    fn move_cursor(&mut self, pressed_key: Key) {
        let Position { mut x, mut y } = self.current_position;
        let width = self.terminal.size().width as usize;
        let height = self.document.len();

        match pressed_key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            _ => (),
        }
        self.current_position = Position { x, y }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::position_cursor(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::position_cursor(&self.current_position);
        }
        Terminal::show_cursor();
        Terminal::flush()
    }
    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    fn draw_row(&self, row: &Row) {
        let start = self.offset.x;
        let width = self.terminal.size().width as usize;
        let end = width + self.offset.x;
        let row = row.render(start, end);
        print!("{}\r", row)
    }
    fn scroll(&mut self) {
        let Position { x, y } = self.current_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                print!("~");
            }
            println!("\r");
        }
    }
}
